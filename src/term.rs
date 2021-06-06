use itertools::{EitherOrBoth, Itertools};
use std::{
    cell::RefCell,
    collections::HashSet,
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
        self.term.hash(state);
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
        true
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
            fn from(mut num: $t) -> Self {
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
}
