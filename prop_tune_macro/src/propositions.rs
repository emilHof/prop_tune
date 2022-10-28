use proc_macro::{Group, Ident, Punct, Span, TokenStream, TokenTree};
use prop_tune_core::operators::{self, Condition, Proposition};

#[derive(Debug, Clone)]
pub struct MacroProp(Proposition);

impl MacroProp {
    pub fn new(prop: impl Into<Proposition>) -> Self {
        MacroProp(prop.into())
    }
}

impl Into<TokenTree> for MacroProp {
    fn into(self) -> TokenTree {
        match self.0 {
            Proposition::Condition(cond) => TokenTree::Ident(match cond {
                Condition::True => Ident::new("true", Span::call_site()),
                Condition::False => Ident::new("false", Span::call_site()),
            }),
            Proposition::Predicate(pred) => {
                TokenTree::Ident(Ident::new(pred.as_str(), Span::call_site()))
            }
            Proposition::Composition(comp) => TokenTree::Group(match *comp {
                operators::Operator::And(a, b) => {
                    let mut stream = TokenStream::new();
                    let toks = vec![
                        MacroProp(a).into(),
                        TokenTree::Punct(Punct::new('&', proc_macro::Spacing::Joint)),
                        TokenTree::Punct(Punct::new('&', proc_macro::Spacing::Alone)),
                        MacroProp(b).into(),
                    ];
                    stream.extend(toks);
                    Group::new(proc_macro::Delimiter::Parenthesis, stream)
                }
                operators::Operator::Or(a, b) => {
                    let mut stream = TokenStream::new();
                    let toks = vec![
                        MacroProp(a).into(),
                        TokenTree::Punct(Punct::new('|', proc_macro::Spacing::Joint)),
                        TokenTree::Punct(Punct::new('|', proc_macro::Spacing::Alone)),
                        MacroProp(b).into(),
                    ];
                    stream.extend(toks);
                    Group::new(proc_macro::Delimiter::Parenthesis, stream)
                }
                operators::Operator::Not(a) => {
                    let mut stream = TokenStream::new();
                    let toks = vec![
                        TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Alone)),
                        MacroProp(a).into(),
                    ];
                    stream.extend(toks);
                    Group::new(proc_macro::Delimiter::None, stream)
                }
                _ => unimplemented!("converting \"implies\" into TokenTree is not supported"),
            }),
        }
    }
}
