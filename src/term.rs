use itertools::{EitherOrBoth, Itertools};
use std::{
    cell::RefCell,
    cmp::{max, Eq, PartialEq},
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Val {
    T, // 1
    F, // 0
    S, // Simplified
}

#[derive(Debug, Clone, Eq)]
pub struct Term {
    pub term: Vec<Val>,
    pub num: u32, // number of `True`s in self.term
    pub sources: Vec<u32>,
    pub flag: RefCell<bool>,
}

impl Term {
    pub fn new(term: Vec<Val>, sources: Vec<u32>, flag: bool) -> Self {
        let num = term.iter().filter(|v| **v == Val::T).count();
        Self {
            term,
            sources,
            num: num as u32,
            flag: RefCell::new(flag),
        }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        let mut diff = 0;
        let length = max(self.term.len(), other.term.len());
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
                let mut new_sources = self.sources.clone();
                new_sources.extend_from_slice(&other.sources);
                Some(Term::new(new_term, new_sources, false))
            }
            _ => None,
        }
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.term
                .iter()
                .map(|val| {
                    match val {
                        Val::T => "1",
                        Val::F => "0",
                        Val::S => "*",
                    }
                })
                .rev()
                .collect::<String>()
        )
    }
}

macro_rules! impl_From_for_Term {
    ($($t:ty)*) => ($(
        impl From<$t> for Term {
            fn from(mut num: $t) -> Self {
                let sources = Vec::from([num as u32]);
                let mut term = Vec::new();
                let mut count = 0;
                while num > 0 {
                    match num & 1 {
                        0 => term.push(Val::F),
                        1 => {
                            term.push(Val::T);
                            count += 1;
                        }
                        _ => unreachable!(),
                    }
                    num >>= 1;
                }

                Self {term, num: count, sources, flag: RefCell::new(false)}
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
                    let expected = Vec::from([Val::F, Val::T, Val::T, Val::T]);
                    assert_eq!(term.term, expected);
                    assert_eq!(term.num, 3);
                }
            }

        )*)
    }

    test_From_for_Term!(u8 u16 u32);
}
