pub use prop_tune_core::{operators, stream};
prop_tune_macro::make_answer!{(a b c) c && !(a && b)}

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn test_macro() {
        println!("{}", answer(true, false, false))
    }
}
