use crate::term::Term;

#[macro_export]
macro_rules! hashset {
    () => {
        HashSet::new();
    };
    ( $($x:expr),+ $(,)?) => {{
        let mut set = HashSet::new();
        $(
            set.insert($x);
        )*
        set
    }};
}

pub fn format_terms<I, T>(terms: I, max_len: usize) -> String
where
    I: IntoIterator<Item = T>,
    T: AsRef<Term>,
{
    let str_terms = terms
        .into_iter()
        .map(|t| format!("{:0>1$}", t.as_ref(), max_len))
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{}]", str_terms)
}
