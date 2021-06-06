use crate::term::Term;

#[macro_export]
macro_rules! hashset {
    ( $($x:expr),* ) => {{
        let mut set = HashSet::new();
        $(
            set.insert($x);
        )*
        set
    }};
}

pub fn format_terms<T: AsRef<Term>>(terms: &[T], max_len: usize) -> String {
    let str_terms = terms
        .iter()
        .map(|t| format!("{:0>1$}", t.as_ref(), max_len))
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{}]", str_terms)
}
