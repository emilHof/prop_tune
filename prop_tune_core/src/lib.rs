pub mod operators;
pub mod stream;

#[cfg(test)]
mod test_lib {
    #[test]
    fn test_macro() {
        prop_tune_macro::make_answer!();

        println!("{}", answer())
    }
}
