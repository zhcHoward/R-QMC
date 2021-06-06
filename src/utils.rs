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
