extern crate proc_macro;
mod propositions;
mod stream;

use proc_macro::{TokenStream, TokenTree};
use prop_tune_core::operators::Proposition;
use propositions::MacroProp;
use stream::PropStream;

#[proc_macro]
pub fn simplify(tok: TokenStream) -> TokenStream {
    let out: PropStream = match tok.try_into() {
        Ok(out) => out,
        Err(err) => {
            panic!("{:?}", err)
        }
    };

    let out: Proposition = match out.try_into() {
        Ok(out) => out,
        Err(err) => panic!("{:?}", err),
    };

    let out = out.simplify();

    let stream = TokenStream::from(Into::<TokenTree>::into(MacroProp::new(out)));
    stream
}
