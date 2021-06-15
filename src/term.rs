use itertools::{repeat_n, EitherOrBoth, Itertools};
use std::{
    cell::RefCell,
    collections::HashSet,
    convert::TryFrom,
    fmt,
    hash::{Hash, Hasher},
};

use crate::hashset;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Val {
    T, // 1
    F, // 0
    S, // Simplified
}

#[derive(Debug, Clone, Eq)]
pub struct Term {
    pub term: Vec<Val>,
    pub sources: HashSet<u32>,
    pub flag: RefCell<bool>,
}

impl Term {
    pub fn new(term: Vec<Val>, sources: HashSet<u32>) -> Self {
        Self {
            term,
            sources,
            flag: RefCell::new(false),
        }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        let mut diff = 0;
        let length = self.term.len().max(other.term.len());
        let mut new_term = Vec::with_capacity(length);
        for pair in self.term.iter().zip_longest(other.term.iter()) {
            let (val1, val2) = match pair {
                EitherOrBoth::Both(val1, val2) => (*val1, *val2),
                EitherOrBoth::Left(val1) => (*val1, Val::F),
                EitherOrBoth::Right(val2) => (Val::F, *val2),
            };
            if val1 != val2 {
                diff += 1;
                new_term.push(Val::S);
            } else {
                new_term.push(val1);
            }
            if diff > 1 {
                break;
            }
        }

        match diff {
            1 => {
                self.flag.replace(true);
                other.flag.replace(true);
                let new_sources = self.sources.union(&other.sources).cloned().collect();
                Some(Term::new(new_term, new_sources))
            }
            _ => None,
        }
    }

    pub fn ones(&self) -> usize {
        self.term.iter().filter(|v| **v == Val::T).count()
    }
}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // If 2 terms share a same source, then these 2 terms are the same.
        // Although, they may be in different form, e.g. "10" and "0010"
        // So, `self.sources` is used instead of `self.term` to calculate the hash value.
        for s in self.sources.iter().sorted() {
            s.hash(state);
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        for pair in self.term.iter().zip_longest(other.term.iter()) {
            let (val1, val2) = match pair {
                EitherOrBoth::Both(val1, val2) => (*val1, *val2),
                EitherOrBoth::Left(val1) => (*val1, Val::F),
                EitherOrBoth::Right(val2) => (Val::F, *val2),
            };
            if val1 != val2 {
                return false;
            }
        }
        self.sources == other.sources
    }
}

impl AsRef<Term> for Term {
    fn as_ref(&self) -> &Term {
        self
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.term
            .iter()
            .map(|val| match val {
                Val::T => "1",
                Val::F => "0",
                Val::S => "*",
            })
            .rev()
            .collect::<String>()
            .fmt(f)
    }
}

macro_rules! impl_From_for_Term {
    ($($t:ty)*) => ($(
        impl From<$t> for Term {
            fn from(num: $t) -> Self {
                let mut num = num;
                let sources = hashset![num as u32];
                let mut term = Vec::new();
                while num > 0 {
                    match num & 1 {
                        0 => term.push(Val::F),
                        1 => term.push(Val::T),
                        _ => unreachable!(),
                    }
                    num >>= 1;
                }
                Self::new(term, sources)
            }
        }
    )*)
}

// u64 and u128 is too large for this algorithm, so ignore them
impl_From_for_Term!(u8 u16 u32);

impl TryFrom<&str> for Term {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut s_count = 0;
        let mut term = value
            .chars()
            .map(|c| match c {
                '1' => Ok(Val::T),
                '0' => Ok(Val::F),
                '*' => {
                    s_count += 1;
                    Ok(Val::S)
                }
                _ => Err("Invalid character in term"),
            })
            .collect::<Result<Vec<Val>, _>>()?;

        let sources = match s_count == 0 {
            true => hashset![u32::from_str_radix(value, 2).unwrap()],
            false => repeat_n(0..2, s_count)
                .multi_cartesian_product()
                .map(|vals| {
                    let mut ivals = vals.into_iter();
                    term.iter()
                        .map(|v| match v {
                            Val::T => 1,
                            Val::F => 0,
                            Val::S => ivals.next().unwrap(),
                        })
                        .fold(0, |acc, v| (acc << 1) + v)
                })
                .collect(),
        };

        term.reverse();
        Ok(Term::new(term, sources))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use paste::paste;

    macro_rules! test_From_for_Term {
        ($($t:ty)*) => ($(
            paste! {
                #[test]
                fn [<test_from_ $t _to_term>]() {
                    let num: $t = 14;
                    let term = Term::from(num);
                    let expected_term = vec![Val::F, Val::T, Val::T, Val::T];
                    let expected_sources = hashset![num as u32];
                    assert_eq!(term.term, expected_term);
                    assert_eq!(term.sources, expected_sources);
                }
            }
        )*)
    }

    test_From_for_Term!(u8 u16 u32);

    #[test]
    fn test_try_form_str_for_term() {
        let s1 = "1*0*";
        let s2 = "1234";
        let s3 = "0010";
        let t1 = Term::try_from(s1);
        let t2 = Term::try_from(s2);
        let t3 = Term::try_from(s3);
        assert!(t1.is_ok());
        assert_eq!(
            t1.unwrap(),
            Term::new(vec![Val::S, Val::F, Val::S, Val::T], hashset![8, 9, 12, 13])
        );
        assert!(t2.is_err());
        assert!(t3.is_ok());
        assert_eq!(t3.unwrap(), Term::new(vec![Val::F, Val::T], hashset![2]))
    }

    #[test]
    fn test_equality_for_term() {
        let t1 = Term::new(vec![Val::T, Val::F, Val::F], hashset![1]);
        let t2 = Term::new(vec![Val::T], hashset![1]);
        assert_eq!(t1, t2);
    }
}
