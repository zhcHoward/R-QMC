use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

mod petrick;
mod term;
use term::Term;

fn main() {
    let minterms: Vec<Term> = vec![4u8, 8, 10, 11, 12, 15]
        .into_iter()
        .map(|num| num.into())
        .collect();
    let not_cares: Vec<Term> = vec![9u8, 14].into_iter().map(|num| num.into()).collect();

    let prime_implicants = find_prime_implicants(&minterms, &not_cares);

    for p in &prime_implicants {
        println!("{}", p);
    }
    println!("");
    // println!("{:?}", prime_implicants);

    let terms = petrick::find_essential_prime_implicants(&prime_implicants, &minterms);
    for p in terms.iter() {
        println!("{}", p);
    }
}

fn find_prime_implicants(minterms: &[Term], not_cares: &[Term]) -> Vec<Term> {
    let mut table = HashMap::new();
    for term in minterms.iter().chain(not_cares.iter()) {
        match table.get_mut(&term.num) {
            None => {
                let mut terms = HashSet::new();
                terms.insert(term.clone());
                table.insert(term.num, terms);
            }
            Some(terms) => {
                terms.insert(term.clone());
            }
        }
    }

    let mut new_table = HashMap::new();
    let mut prime_implicants = Vec::new();
    let mut new_implicants = true;
    while new_implicants {
        new_implicants = false;
        for key in table.keys().sorted() {
            let terms1 = table.get(key).unwrap();
            if let Some(terms2) = table.get(&(key + 1)) {
                for (t1, t2) in iproduct!(terms1, terms2) {
                    // print!("{} + {}", t1, t2);
                    match t1.combine(&t2) {
                        None => {
                            // println!(" = None");
                            continue;
                        }
                        Some(new_term) => {
                            // println!(" = {}", new_term);
                            match new_table.get_mut(key) {
                                None => {
                                    let mut new_terms = HashSet::new();
                                    new_terms.insert(new_term);
                                    new_table.insert(*key, new_terms);
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

            for term in terms1.iter() {
                if !*term.flag.borrow() {
                    prime_implicants.push(term.clone());
                }
            }
        }
        table = new_table.clone();
        new_table.clear();
    }
    prime_implicants
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::term::Val;
    use std::iter::FromIterator;

    #[test]
    fn test_find_prime_implicants_with_not_care() {
        let minterms: Vec<Term> = vec![4u8, 8, 10, 11, 12, 15]
            .into_iter()
            .map(|num| num.into())
            .collect();
        let not_cares: Vec<Term> = vec![9u8, 14].into_iter().map(|num| num.into()).collect();
        let prime_implicants = find_prime_implicants(&minterms, &not_cares);
        #[rustfmt::skip]
        let expected = vec![
            Term::new(vec![Val::F, Val::F, Val::T, Val::S], vec![4, 12], false),
            Term::new(vec![Val::F, Val::S, Val::S, Val::T], vec![8, 10, 12, 14], false),
            Term::new(vec![Val::S, Val::S, Val::F, Val::T], vec![8, 10, 9, 11], false),
            Term::new(vec![Val::S, Val::T, Val::S, Val::T], vec![10, 11, 14, 15], false),
        ];
        assert_eq!(
            HashSet::<_>::from_iter(prime_implicants.into_iter()),
            HashSet::<_>::from_iter(expected.into_iter())
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
        let expected = vec![
            Term::new(vec![Val::F, Val::T], vec![2], false),
            Term::new(vec![Val::T, Val::F, Val::F, Val::S], vec![1, 9], false),
            Term::new(vec![Val::T, Val::S, Val::F, Val::T], vec![9, 11], false),
            Term::new(vec![Val::F, Val::S, Val::T, Val::T], vec![12, 14], false),
            Term::new(vec![Val::S, Val::T, Val::T, Val::T], vec![14, 15], false),
            Term::new(vec![Val::T, Val::T, Val::S, Val::T], vec![11, 15], false),
        ];
        assert_eq!(
            HashSet::<_>::from_iter(prime_implicants.into_iter()),
            HashSet::<_>::from_iter(expected.into_iter())
        );
    }
}
