use proc_macro::{TokenStream, TokenTree};
use prop_tune_core::operators::Proposition;
use prop_tune_core::stream::{self, Bracket, Operator, Token};

#[derive(Debug, Clone)]
pub struct PropStream(stream::TokenStream);

impl PropStream {
    pub fn new(stream: impl Into<stream::TokenStream>) -> Self {
        PropStream(stream.into())
    }
}

impl Into<stream::TokenStream> for PropStream {
    fn into(self) -> stream::TokenStream {
        self.0
    }
}

impl TryInto<Proposition> for PropStream {
    type Error = <stream::TokenStream as TryInto<Proposition>>::Error;
    fn try_into(self) -> Result<Proposition, Self::Error> {
        self.0.try_into()
    }
}

impl TryInto<PropStream> for TokenStream {
    type Error = String;
    fn try_into(self) -> Result<PropStream, Self::Error> {
        let tok: Vec<TokenTree> = self.into_iter().collect();
        let mut out = PropStream(stream::TokenStream(vec![]));

        let mut i = 0;
        while i < tok.len() {
            match &tok[i] {
                TokenTree::Punct(ident) => match ident.to_string().as_str() {
                    "&" => match &tok[i + 1] {
                        TokenTree::Punct(ident2) => match ident2.to_string().as_str() {
                            "&" => {
                                out.0 .0.push(Token::Operator(Operator::And));
                                i += 2
                            }
                            _ => return Err("&* is not a valid operator".to_string()),
                        },
                        _ => return Err("Invalid operator".to_string()),
                    },
                    "|" => match &tok[i + 1] {
                        TokenTree::Punct(ident2) => match ident2.to_string().as_str() {
                            "|" => {
                                out.0 .0.push(Token::Operator(Operator::Or));
                                i += 2
                            }
                            _ => return Err("|* is not a valid operator".to_string()),
                        },
                        _ => return Err("Invalid operator".to_string()),
                    },
                    "!" => {
                        out.0 .0.push(Token::Operator(Operator::Not));
                        i += 1
                    }
                    _ => return Err("Invalid punctuation".to_string()),
                },
                TokenTree::Ident(ident) => {
                    out.0 .0.push(Token::Predicate(ident.to_string()));
                    i += 1;
                }
                TokenTree::Group(inner_tok) => {
                    let mut inner_out: PropStream = match inner_tok.stream().try_into() {
                        Ok(inner_out) => inner_out,
                        Err(err) => return Err(err),
                    };
                    out.0 .0.push(Token::Bracket(Bracket::Open));
                    out.0 .0.append(&mut inner_out.0 .0);
                    out.0 .0.push(Token::Bracket(Bracket::Close));
                    i += 1;
                }

                TokenTree::Literal(_) => return Err("No implimentation for literals!".to_string()),
            }
        }
        Ok(out)
    }
}
