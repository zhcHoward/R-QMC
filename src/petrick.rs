use std::{
    collections::{hash_set::Iter, HashMap, HashSet},
    hash::{Hash, Hasher},
    mem,
};

use itertools::{iproduct, Itertools};
use log::debug;

use crate::{hashset, term::Term};

pub fn find_essential_prime_implicants<'a>(
    prime_implicants: &'a [Term],
    minterms: &[Term],
) -> Vec<&'a Term> {
    // generate prime implicant chart
    let mut prime_implicant_chart = HashMap::new();
    for key in minterms.iter().flat_map(|term| term.sources.iter()) {
        prime_implicant_chart.insert(key, HashSet::new());
    }
    for (id, implicant) in prime_implicants.iter().enumerate() {
        for source in implicant.sources.iter() {
            match prime_implicant_chart.get_mut(source) {
                None => continue,
                Some(value) => {
                    value.insert(id);
                }
            }
        }
    }

    // multiply all the sums in prime implicant chart without any simplification
    let mut sop = SumOfProduct::new();
    for value in prime_implicant_chart.values() {
        sop.multiply(value);
    }
    debug!("sum of products: {:?}", sop);

    // find 1 shortest product among all the products
    let mut min_len = usize::MAX;
    let mut ids = Vec::new();
    for p in sop.iter() {
        let length = p.len();
        if length < min_len {
            min_len = length;
            ids = p.iter().collect();
        }
    }

    // collect all terms by their indexes in prime implicant list
    prime_implicants
        .iter()
        .enumerate()
        .filter(|(id, _)| ids.contains(&id))
        .map(|(_, term)| term)
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Product(HashSet<usize>);

impl Product {
    fn new(i: usize) -> Self {
        Self(hashset![i])
    }

    fn multiply(&self, i: usize) -> Product {
        let mut result = self.clone();
        result.0.insert(i);
        result
    }

    fn iter(&self) -> Iter<usize> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.0.iter().sorted() {
            i.hash(state);
        }
    }
}

#[derive(Debug)]
struct SumOfProduct(HashSet<Product>);

impl SumOfProduct {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn multiply(&mut self, sum: &HashSet<usize>) {
        if self.0.is_empty() {
            for id in sum {
                self.0.insert(Product::new(*id));
            }
        } else {
            let mut result = HashSet::new();
            for (p, id) in iproduct!(&self.0, sum) {
                let new_p = p.multiply(*id);
                result.insert(new_p);
            }
            mem::swap(self, &mut Self(result));
        }
    }

    fn iter(&self) -> Iter<Product> {
        self.0.iter()
    }
}

impl Hash for SumOfProduct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for p in &self.0 {
            p.hash(state);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::term::Val;

    #[test]
    fn test_find_essential_prime_implicants() {
        let minterms: Vec<Term> = vec![4u8, 8, 10, 11, 12, 15]
            .into_iter()
            .map(|num| num.into())
            .collect();
        #[rustfmt::skip]
        let pi = vec![
            Term::new(vec![Val::F, Val::F, Val::T, Val::S], hashset![4, 12]),
            Term::new(vec![Val::F, Val::S, Val::S, Val::T], hashset![8, 10, 12, 14]),
            Term::new(vec![Val::S, Val::S, Val::F, Val::T], hashset![8, 10, 9, 11]),
            Term::new(vec![Val::S, Val::T, Val::S, Val::T], hashset![10, 11, 14, 15]),
        ];
        let expected1 = vec![&pi[0], &pi[1], &pi[3]];
        let expected2 = vec![&pi[0], &pi[2], &pi[3]];
        let result = find_essential_prime_implicants(&pi, &minterms);
        assert!(result == expected1 || result == expected2);
    }
}
