use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    iter::FromIterator,
    mem,
};

use itertools::Itertools;

use crate::term::Term;

pub fn find_essential_prime_implicants<'a>(
    prime_implicants: &'a [Term],
    minterms: &[Term],
) -> Vec<&'a Term> {
    let mut map = HashMap::new();
    for key in minterms.iter().map(|term| term.sources[0]) {
        map.insert(key, HashSet::new());
    }
    for (id, implicant) in prime_implicants.iter().enumerate() {
        for source in implicant.sources.iter() {
            match map.get_mut(source) {
                None => continue,
                Some(value) => {
                    value.insert(id);
                }
            }
        }
    }

    let mut result = SumOfProduct::new();
    for value in map.values() {
        result.multiply(value);
    }

    let mut min_len = usize::MAX;
    let mut ids = vec![];
    for p in result.0.iter() {
        let length = p.0.len();
        if length < min_len {
            min_len = length;
            ids = Vec::from_iter(p.0.iter().map(|v| *v))
        }
    }

    prime_implicants
        .iter()
        .enumerate()
        .filter(|(id, _)| ids.contains(id))
        .map(|a| a.1)
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Product(HashSet<usize>);

impl Product {
    fn new(i: usize) -> Self {
        let mut set = HashSet::new();
        set.insert(i);
        Self(set)
    }

    fn multiply(&self, i: usize) -> Product {
        let mut result = self.clone();
        result.0.insert(i);
        result
    }
}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.0.iter().sorted() {
            i.hash(state);
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

struct SumOfProduct(HashSet<Product>);

impl SumOfProduct {
    fn new() -> Self {
        Default::default()
    }

    fn multiply(&mut self, bs: &HashSet<usize>) {
        if self.0.is_empty() {
            for b in bs {
                let p = Product::new(*b);
                self.0.insert(p);
            }
        } else {
            let mut result = HashSet::new();
            for p in &self.0 {
                for i in bs {
                    let new_p = p.multiply(*i);
                    result.insert(new_p);
                }
            }
            mem::swap(self, &mut Self(result));
        }
    }
}

impl Hash for SumOfProduct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for p in &self.0 {
            p.hash(state);
        }
    }
}

impl Default for SumOfProduct {
    fn default() -> Self {
        Self(HashSet::new())
    }
}