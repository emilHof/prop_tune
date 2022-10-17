extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Ident, Group, Punct, Span};
use prop_tune_core::{stream::{self, Bracket, Operator, Token}, operators::{self, Proposition, Condition}};
use quote::quote;

#[allow(dead_code)]
#[derive(Debug)]
struct PropStream(stream::TokenStream);

impl TryInto<PropStream> for TokenStream {
    type Error = String;
    fn try_into(self) -> Result<PropStream, Self::Error> {
        let tok: Vec<TokenTree> = self.into_iter().collect();
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
                    "!" => {
                        out.0.0.push(Token::Operator(Operator::Not));
                        i += 1
                    },
                    _ => return Err("Invalid punctuation".to_string()),
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

#[allow(dead_code)]
#[derive(Debug)]
struct MacroProp(Proposition);

impl Into<TokenTree> for MacroProp {
    
    fn into(self) -> TokenTree {
        match self.0 {
            Proposition::Condition(cond) => TokenTree::Ident(match cond {
                Condition::True => Ident::new("True", Span::call_site()),
                Condition::False => Ident::new("False", Span::call_site()),
            }),
            Proposition::Predicate(pred) => TokenTree::Ident(Ident::new(pred.as_str(), Span::call_site())),
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
                },
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
                },
                operators::Operator::Not(a) => {
                    let mut stream = TokenStream::new();
                    let toks = vec![
                        TokenTree::Punct(Punct::new('!', proc_macro::Spacing::Alone)), 
                        MacroProp(a).into(),
                    ];
                    stream.extend(toks);
                    Group::new(proc_macro::Delimiter::None, stream)
                },
                _ => unimplemented!("converting \"implies\" into TokenTree is not supported"),
            })

        }
    }
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    let mut tok = _item.into_iter().collect::<Vec<TokenTree>>();
    let params = TokenStream::from_iter(match &tok[0] {
        TokenTree::Group(group) => group.stream().into_iter().fold(vec![], |mut acc, ident| { 
            acc.append(&mut vec![
                    ident, 
                    TokenTree::Punct(Punct::new(':', proc_macro::Spacing::Alone)),
                    TokenTree::Ident(Ident::new("bool", proc_macro::Span::call_site())), 
                    TokenTree::Punct(Punct::new(',', proc_macro::Spacing::Alone))
            ]);
            acc
        }),
        _ => panic!("expected parameters to be declared"),

    });

    tok.remove(0);

    eprintln!("params: {:?}", params);

    let out: PropStream = match TokenStream::from_iter(tok).try_into() {
        Ok(out) => out,
        Err(err) => {panic!("{}", err)},
    };

    eprintln!("PropStream: {:?}", out);

    let out: Proposition = match out.0.try_into() {
        Ok(out) => out,
        Err(err) => panic!("{:?}", err),
    };

    let out = out.simplify();

    let stream = TokenStream::from_iter(
        vec![
            TokenTree::Ident(Ident::new("fn", Span::call_site())),
            TokenTree::Ident(Ident::new("answer", Span::call_site())),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, params)),
            TokenTree::Punct(Punct::new('-', proc_macro::Spacing::Joint)),
            TokenTree::Punct(Punct::new('>', proc_macro::Spacing::Alone)),
            TokenTree::Ident(Ident::new("bool", Span::call_site())),
            TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, TokenStream::from_iter(
                vec![
                    TokenTree::Ident(Ident::new("if", Span::call_site())),
                    MacroProp(out).into(),
                    TokenTree::Group(
                        Group::new(
                            proc_macro::Delimiter::Brace, 
                            TokenStream::from(TokenTree::Ident(Ident::new("true", Span::call_site())))
                        )
                    ),
                    TokenTree::Ident(Ident::new("else", Span::call_site())),
                    TokenTree::Group(
                        Group::new(
                            proc_macro::Delimiter::Brace, 
                            TokenStream::from(TokenTree::Ident(Ident::new("false", Span::call_site())))
                        )
                    ),
                ]
            )))
        ], 
    );
    stream
}

