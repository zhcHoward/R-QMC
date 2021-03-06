use std::collections::{HashMap, HashSet};

use itertools::{iproduct, Itertools};
use log::debug;

use crate::{hashset, term::Term};

pub fn find_prime_implicants(minterms: &[Term], not_cares: &[Term]) -> Vec<Term> {
    let mut table = HashMap::new();
    for term in minterms.iter().chain(not_cares.iter()) {
        let ones = term.ones();
        match table.get_mut(&ones) {
            None => {
                table.insert(ones, hashset![term.clone()]);
            }
            Some(terms) => {
                terms.insert(term.clone());
            }
        }
    }

    let mut prime_implicants = Vec::new();
    let mut new_implicants = true;
    while new_implicants {
        new_implicants = false;
        let mut new_table = HashMap::new();
        for key in table.keys().sorted() {
            let terms1 = table.get(key).unwrap();
            match table.get(&(key + 1)) {
                None => debug!("for key == {}, terms2 not exist, skip", key),
                Some(terms2) => {
                    debug!(
                        "for key == {}, terms2 found, trying to combine terms...",
                        key
                    );
                    for (t1, t2) in iproduct!(terms1, terms2) {
                        match t1.combine(&t2) {
                            None => {
                                debug!("{} + {} => None", t1, t2);
                                continue;
                            }
                            Some(new_term) => {
                                debug!("{} + {} => {}", t1, t2, new_term);
                                match new_table.get_mut(key) {
                                    None => {
                                        new_table.insert(*key, hashset!(new_term));
                                    }
                                    Some(terms) => {
                                        terms.insert(new_term);
                                    }
                                };
                                new_implicants = true;
                            }
                        }
                    }
                }
            }

            for term in terms1.iter() {
                if !*term.flag.borrow() {
                    prime_implicants.push(term.clone());
                    debug!("{} become prime implicant", term);
                }
            }
        }
        table = new_table;
    }
    prime_implicants
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    fn test_find_prime_implicants_with_not_care() {
        let minterms: Vec<Term> = vec![4u8, 8, 10, 11, 12, 15]
            .into_iter()
            .map(|num| num.into())
            .collect();
        let not_cares: Vec<Term> = vec![9u8, 14].into_iter().map(|num| num.into()).collect();
        let prime_implicants = find_prime_implicants(&minterms, &not_cares);
        let expected = hashset![
            Term::try_from("*100").unwrap(),
            Term::try_from("1**0").unwrap(),
            Term::try_from("10**").unwrap(),
            Term::try_from("1*1*").unwrap(),
        ];
        assert_eq!(
            prime_implicants.into_iter().collect::<HashSet<Term>>(),
            expected
        );
    }

    #[test]
    fn test_find_prime_implicants_without_not_care() {
        let minterms: Vec<Term> = vec![1u8, 2, 9, 11, 12, 14, 15]
            .into_iter()
            .map(|num| num.into())
            .collect();
        let not_cares: Vec<Term> = Vec::new();
        let prime_implicants = find_prime_implicants(&minterms, &not_cares);
        let expected = hashset![
            Term::try_from("0010").unwrap(),
            Term::try_from("*001").unwrap(),
            Term::try_from("10*1").unwrap(),
            Term::try_from("11*0").unwrap(),
            Term::try_from("111*").unwrap(),
            Term::try_from("1*11").unwrap(),
        ];
        assert_eq!(
            prime_implicants.into_iter().collect::<HashSet<Term>>(),
            expected
        );
    }
}
