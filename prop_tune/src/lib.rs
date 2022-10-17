pub use prop_tune_core::{operators, stream};
pub use prop_tune_macro::simplify;

fn eval(a: bool, b: bool, c: bool, d: bool) -> bool {
    if simplify!{c && !(a && (!c && d)) && b} {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;

    #[test]
    fn test_macro() {
        println!("{}", eval(true, true, false, false))
    }
}
