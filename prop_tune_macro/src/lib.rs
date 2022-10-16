extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use prop_tune_core::{stream::{self, Bracket, Operator, Token}, operators::Proposition};
use quote::quote;

#[allow(dead_code)]
#[derive(Debug)]
struct PropStream(stream::TokenStream);

impl TryInto<PropStream> for TokenStream {
    type Error = String;
    fn try_into(self) -> Result<PropStream, Self::Error> {
        let tok: Vec<TokenTree> = self.into_iter().collect();
        eprintln!("{:?}", tok[0]);
        let mut out = PropStream(stream::TokenStream(vec![]));

        let mut i = 0;
        while i < tok.len() {
            match &tok[i] {
                TokenTree::Punct(ident) => match ident.to_string().as_str() {
                    "&" => match &tok[i+1] {
                        TokenTree::Punct(ident2) => match ident2.to_string().as_str() {
                            "&" => {
                                out.0.0.push(Token::Operator(Operator::And));
                                i += 2
                            },
                            _ => return Err("&* is not a valid operator".to_string())
                        },
                        _ => return Err("Invalid operator".to_string())
                    },
                    "|" => match &tok[i+1] {
                        TokenTree::Punct(ident2) => match ident2.to_string().as_str() {
                            "|" => {
                                out.0.0.push(Token::Operator(Operator::And));
                                i += 2
                            },
                            _ => return Err("|* is not a valid operator".to_string())
                        },
                        _ => return Err("Invalid operator".to_string())
                    },
                    _ => return Err("Invalid punctuation".to_string())
                },
                TokenTree::Ident(ident) => {
                    out.0.0.push(Token::Predicate(ident.to_string()));
                    i += 1;
                },
                TokenTree::Group(inner_tok) => {
                    let mut inner_out: PropStream = match inner_tok.stream().try_into() {
                        Ok(inner_out) => inner_out,
                        Err(err) => return Err(err),
                    };
                    out.0.0.push(Token::Bracket(Bracket::Open));
                    out.0.0.append(&mut inner_out.0.0);
                    out.0.0.push(Token::Bracket(Bracket::Close));
                    i += 1;
                }

                TokenTree::Literal(_) => return Err("No implimentation for literals!".to_string())
            }
        }
        Ok(out)
    }
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {

    let out: PropStream = match _item.try_into() {
        Ok(out) => out,
        Err(err) => {panic!("{}", err)},
    };

    eprintln!("{:?}", out);

    let out: Proposition = match out.0.try_into() {
        Ok(out) => out,
        Err(err) => panic!("{:?}", err),
    };

    eprintln!("{:?}", out);

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
