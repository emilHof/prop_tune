pub use prop_tune_core::{operators, stream};

#[cfg(test)]
mod test_lib {
    #[test]
    fn test_macro() {
        prop_tune_macro::make_answer!{(a && b)};

        println!("{}", answer())
    }
}
