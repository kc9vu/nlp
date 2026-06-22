mod elder;
pub use elder::natural_cmp as natural_quick_cmp;

// use std::borrow::Cow;
use std::cmp::Ordering::{self, *};

#[cfg(feature = "bigint")]
use num_bigint::BigUint;
use winnow::ascii::digit1;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;
use winnow::stream::AsChar;
use winnow::token::{rest, take_while};

pub fn natural_cmp<S>(left: S, right: S) -> Ordering
where
    S: AsRef<str>,
{
    let mut left = left.as_ref();
    let mut right = right.as_ref();

    while !left.is_empty() && !right.is_empty() {
        let ta = parse_token(&mut left).unwrap();
        let tb = parse_token(&mut right).unwrap();
        match ta.cmp(&tb) {
            Equal => continue,
            ord => return ord,
        }
    }
    left.len().cmp(&right.len())
}

pub fn natural_sort_key<'s>(s: &'s str) -> CmpState<'s> {
    CmpState::parse(s)
}

pub fn natural_cmp_filename<S>(left: S, right: S) -> Ordering
where
    S: AsRef<str>,
{
    let left = left.as_ref();
    let right = right.as_ref();

    if let Ok((left_name, left_stem)) = parse_filename.parse(left)
        && let Ok((right_name, right_stem)) = parse_filename.parse(right)
        && left_stem == right_stem
    {
        CmpToken::Str(left_name).cmp(&CmpToken::Str(right_name))
    } else {
        natural_cmp(left, right)
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CmpState<'s> {
    tokens: Vec<CmpToken<'s>>,
}

impl<'s> CmpState<'s> {
    fn parse(s: &'s str) -> Self {
        // let mut input = s.str();
        // alt((parse_hex,)).parse(s).unwrap()
        let tokens: Vec<CmpToken<'s>> = repeat(1.., parse_token)
            .parse(s)
            .unwrap_or_else(|_| vec![CmpToken::Str(s)]);
        Self { tokens }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq)]
enum CmpToken<'s> {
    Str(&'s str),
    Int(u64),
    #[cfg(not(feature = "bigint"))]
    Numeric(&'s str),
    #[cfg(feature = "bigint")]
    BigInt(BigUint),
}

#[cfg(feature = "bigint")]
fn parse_digit<'s>(input: &mut &'s str) -> ModalResult<CmpToken<'s>> {
    digit1
        .try_map(|s: &str| {
            if let Ok(i) = s.parse::<u64>() {
                Ok(CmpToken::Int(i))
            } else {
                s.parse::<BigUint>().map(CmpToken::BigInt)
            }
        })
        .parse_next(input)
}

#[cfg(not(feature = "bigint"))]
fn parse_digit<'s>(input: &mut &'s str) -> ModalResult<CmpToken<'s>> {
    digit1.parse_next(input).map(|s| {
        if let Ok(i) = s.parse::<u64>() {
            CmpToken::Int(i)
        } else {
            CmpToken::Numeric(s)
        }
    })
}

fn parse_non_digits<'s>(input: &mut &'s str) -> ModalResult<CmpToken<'s>> {
    take_while(1.., |c| !AsChar::is_dec_digit(c))
        .parse_next(input)
        .map(CmpToken::Str)
}

fn parse_token<'s>(input: &mut &'s str) -> ModalResult<CmpToken<'s>> {
    alt((parse_digit, parse_non_digits)).parse_next(input)
}

fn parse_filename<'s>(input: &mut &'s str) -> ModalResult<(&'s str, &'s str)> {
    (take_while(1.., |c| c != '.'), rest).parse_next(input)
}

impl<'s> PartialOrd for CmpToken<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'s> Ord for CmpToken<'s> {
    fn cmp(&self, other: &Self) -> Ordering {
        use CmpToken::*;

        match (self, other) {
            (Str(left), Str(right)) => left.cmp(right),
            (Str(_), _) => Greater,
            (_, Str(_)) => Less,
            #[cfg(not(feature = "bigint"))]
            (Numeric(left), Numeric(right)) if left.len() == right.len() => left.cmp(right),
            (Numeric(left), Numeric(right)) => left.len().cmp(&right.len()),
            #[cfg(not(feature = "bigint"))]
            (Numeric(_), _) => Greater,
            #[cfg(not(feature = "bigint"))]
            (_, Numeric(_)) => Less,
            (Int(left), Int(right)) => left.cmp(right),
            #[cfg(feature = "bigint")]
            (Int(_), BigInt(_)) => Less,
            #[cfg(feature = "bigint")]
            (BigInt(left), BigInt(right)) => left.cmp(right),
            #[cfg(feature = "bigint")]
            (BigInt(_), Int(_)) => Greater,
        }
    }
}
