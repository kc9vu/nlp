//! Natural sorting for Rust strings,
//! from https://github.com/jwiesler/natural-sort-rs/

use std::borrow::Cow;
use std::cmp::Ordering::{self, *};

#[cfg(feature = "bigint")]
use num_bigint::BigUint;

pub fn natural_cmp(mut a: &str, mut b: &str) -> Ordering {
    while !a.is_empty() && !b.is_empty() {
        let chunk_a = natural_chunk(a);
        let chunk_b = &natural_chunk(b);

        match chunk_a.cmp(chunk_b) {
            Equal => (),
            ord => return ord,
        }

        a = &a[chunk_a.chars.len()..];
        b = &b[chunk_b.chars.len()..];
    }

    a.len().cmp(&b.len())
}

pub fn natural_sort<S>(s: S) -> CmpState<'static>
where
    S: Into<CmpState<'static>>,
{
    s.into()
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CmpState<'s> {
    chunks: Vec<Chunk<'s>>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq)]
struct Chunk<'s> {
    chars: Cow<'s, str>,
    are_digits: bool,
}

impl<'s> Chunk<'s> {
    pub fn new<S>(chars: S, are_digits: bool) -> Self
    where
        S: Into<Cow<'s, str>>,
    {
        Chunk {
            chars: chars.into(),
            are_digits,
        }
    }

    #[cfg(not(feature = "bigint"))]
    fn parse_to_int(&self) -> Option<u64> {
        self.are_digits.then(|| self.chars.parse().ok()).flatten()
    }

    #[cfg(feature = "bigint")]
    fn parse_to_int(&self) -> Option<BigUint> {
        self.are_digits.then(|| self.chars.parse().ok()).flatten()
    }
}

fn natural_chunk(a: &str) -> Chunk<'_> {
    let is_digit = match a.as_bytes().first() {
        None => return Chunk::new(a, false),
        Some(c) => c.is_ascii_digit(),
    };
    for (i, c) in a.bytes().enumerate() {
        if c.is_ascii_digit() != is_digit {
            return Chunk::new(&a[..i], is_digit);
        }
    }
    Chunk::new(a, is_digit)
}

impl<'s, S> From<S> for CmpState<'static>
where
    S: Into<Cow<'s, str>>,
{
    fn from(value: S) -> Self {
        let s = value.into();
        let mut st = s.as_ref();
        let mut chunks = vec![];
        while !st.is_empty() {
            let c = natural_chunk(st);
            st = &st[c.chars.len()..];
            chunks.push(Chunk::new(c.chars.to_string(), c.are_digits));
        }
        Self { chunks }
    }
}

impl<'s> PartialOrd for Chunk<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'s> Ord for Chunk<'s> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.parse_to_int(), other.parse_to_int()) {
            (Some(ia), Some(ib)) => ia.cmp(&ib),
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (None, None) => self.chars.cmp(&other.chars),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_cmp_works() {
        assert_eq!(Ordering::Equal, natural_cmp("", ""));
        assert_eq!(Ordering::Equal, natural_cmp("1", "1"));
        assert_eq!(Ordering::Equal, natural_cmp("a", "a"));

        assert_eq!(Ordering::Less, natural_cmp("ab", "ac"));
        assert_eq!(Ordering::Greater, natural_cmp("ac", "ab"));

        assert_eq!(Ordering::Less, natural_cmp("test1", "test12"));
        assert_eq!(Ordering::Greater, natural_cmp("test12", "test1"));

        assert_eq!(Ordering::Less, natural_cmp("1", "2"));
        assert_eq!(Ordering::Greater, natural_cmp("2", "1"));

        assert_eq!(Ordering::Less, natural_cmp("12", "13"));
        assert_eq!(Ordering::Greater, natural_cmp("13", "12"));

        assert_eq!(Ordering::Less, natural_cmp("1a", "12"));
        assert_eq!(Ordering::Greater, natural_cmp("12", "1a"));

        assert_eq!(Ordering::Greater, natural_cmp("aa", "a2"));
        assert_eq!(Ordering::Less, natural_cmp("a2", "aa"));

        assert_eq!(Ordering::Greater, natural_cmp("a", "1"));
        assert_eq!(Ordering::Less, natural_cmp("1", "a"));

        assert_eq!(Ordering::Less, natural_cmp("2", "12"));
        assert_eq!(Ordering::Greater, natural_cmp("12", "2"));

        assert_eq!(Ordering::Less, natural_cmp("12a", "22b"));
        assert_eq!(Ordering::Greater, natural_cmp("22b", "12a"));

        assert_eq!(Ordering::Less, natural_cmp("12a", "221"));
        assert_eq!(Ordering::Greater, natural_cmp("221", "12a"));

        assert_eq!(Ordering::Less, natural_cmp("06", "8"));
        assert_eq!(Ordering::Greater, natural_cmp("8", "06"));
    }
}
