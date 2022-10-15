extern crate proc_macro;
use proc_macro::TokenStream;
use prop_tune_core::operators::{Proposition};
use quote::quote;

#[allow(dead_code)]
struct Composite {
    props: Proposition,
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {

    eprintln!("hi! ");

    quote!{
        fn answer() -> prop_tune_core::operators::Proposition {
            use prop_tune_core::operators::Proposition;
            Proposition::new_and("A", Proposition::new_not("B"))
        }
    }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
