/// Natural sorting for Rust strings,
/// from https://github.com/jwiesler/natural-sort-rs/

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
struct Chunk<'a> {
    chars: &'a str,
    are_digits: bool,
}

impl<'a> Chunk<'a> {
    pub fn new(chars: &'a str, are_digits: bool) -> Self {
        Chunk { chars, are_digits }
    }
}

fn natural_chunk(a: &str) -> Chunk {
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

pub fn natural_cmp(mut a: &str, mut b: &str) -> Ordering {
    while !a.is_empty() && !b.is_empty() {
        let chunk_a = natural_chunk(a);
        let chunk_b = natural_chunk(b);

        if chunk_a.are_digits && chunk_b.are_digits {
            /* match chunk_a.chars.len().cmp(&chunk_b.chars.len()) */
            match chunk_a
                .chars
                .parse::<u32>()
                .unwrap()
                .cmp(&chunk_b.chars.parse::<u32>().unwrap())
            {
                Ordering::Equal => (),
                v => return v,
            }
        }

        match chunk_a.chars.cmp(chunk_b.chars) {
            Ordering::Equal => (),
            v => return v,
        }

        a = &a[chunk_a.chars.len()..];
        b = &b[chunk_b.chars.len()..];
    }

    a.len().cmp(&b.len())
}

pub fn natural_sort<S: AsRef<str>>(items: &[S]) -> Vec<&str> {
    let mut items = items.into_iter().map(AsRef::as_ref).collect::<Vec<_>>();
    items.sort_by(|a, b| natural_cmp(a, b));
    items
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

    #[test]
    fn natural_sort_works() {
        assert_eq!(vec!["3", "8z", "12", "12n", "34n", "one", "two"], natural_sort(&["one", "two", "3", "12", "34n", "12n", "8z"]));
    }
}
