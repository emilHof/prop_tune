mod methods;

use crate::stream;

#[derive(Debug, Clone, Hash)]
pub enum Operator {
    And(Proposition, Proposition),
    Or(Proposition, Proposition),
    Implies(Proposition, Proposition),
    Not(Proposition),
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Operator::And(a, b) => match other {
                Operator::And(c, d) => (a.eq(c) && b.eq(d)) || (a.eq(d) && b.eq(c)),
                _ => false,
            },
            Operator::Or(a, b) => match other {
                Operator::Or(c, d) => (a.eq(c) && b.eq(d)) || (a.eq(d) && b.eq(c)),
                _ => false,
            },
            Operator::Implies(a, b) => match other {
                Operator::Implies(c, d) => a.eq(c) && b.eq(d),
                _ => false,
            },
            Operator::Not(a) => match other {
                Operator::Not(b) => a.eq(b),
                _ => false,
            },
        }
    }
}

impl Eq for Operator {}

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Proposition {
    Condition(Condition),
    Predicate(String),
    Composition(Box<Operator>),
}

impl Proposition {
    pub fn new_and<A: Into<Proposition>, B: Into<Proposition>>(a: A, b: B) -> Proposition {
        Proposition::Composition(Box::new(Operator::And(a.into(), b.into())))
    }

    pub fn new_or<A: Into<Proposition>, B: Into<Proposition>>(a: A, b: B) -> Proposition {
        Proposition::Composition(Box::new(Operator::Or(a.into(), b.into())))
    }

    pub fn new_not<A: Into<Proposition>>(a: A) -> Proposition {
        Proposition::Composition(Box::new(Operator::Not(a.into())))
    }

    pub fn new_implies<A: Into<Proposition>, B: Into<Proposition>>(a: A, b: B) -> Proposition {
        Proposition::Composition(Box::new(Operator::Implies(a.into(), b.into())))
    }

    pub fn new_pred<A: Into<String>>(a: A) -> Proposition {
        Proposition::Predicate(a.into())
    }

    pub fn new_true() -> Proposition {
        Proposition::Condition(Condition::True)
    }

    pub fn new_false() -> Proposition {
        Proposition::Condition(Condition::False)
    }
}

impl Eq for Proposition {}

impl Into<Proposition> for &str {
    fn into(self) -> Proposition {
        Proposition::Predicate(self.into())
    }
}

impl Into<Proposition> for String {
    fn into(self) -> Proposition {
        Proposition::Predicate(self)
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Condition {
    True,
    False,
}

impl Eq for Condition {}

impl TryInto<Proposition> for crate::stream::TokenStream {
    type Error = ParseError;

    fn try_into(self) -> Result<Proposition, Self::Error> {
        if self.len() < 1 {
            return Err(ParseError);
        }

        let mut i = 0;

        parse_prop(&mut i, &self)
    }
}

fn flip_prop(stream: &mut stream::TokenStream) {
    // TODO: Deal with flipping the nots properly
    // Right now the cases of nots in from of parens is not handled correctly
    stream.0 = stream.0.clone().into_iter().rev().collect();
    for i in 0..stream.len() {
        match &stream[i] {
            stream::Token::Bracket(br) => {
                stream[i] = stream::Token::Bracket(match br {
                    stream::Bracket::Open => stream::Bracket::Close,
                    stream::Bracket::Close => stream::Bracket::Open,
                })
            }
            _ => (),
        }
    }

    for i in 0..stream.len() {
        match &stream[i] {
            stream::Token::Operator(stream::Operator::Not) => {
                let mut i = i;
                // count the parentheses to make sure the negation is placed in the right place
                let mut parens = 0;
                while let Some(j) = i.checked_sub(1) {
                    parens += if stream[j] == stream::Token::Bracket(stream::Bracket::Close) {
                        1
                    } else if stream[j] == stream::Token::Bracket(stream::Bracket::Open) {
                        -1
                    } else {
                        0
                    };

                    unsafe {
                        let stream = stream as *mut stream::TokenStream;
                        std::mem::swap(&mut (*stream)[i], &mut (*stream)[j])
                    }

                    i = j;

                    // break if the end of the parentheses block is reached
                    if parens == 0 {
                        break;
                    }
                }
            }
            _ => (),
        }
    }
}

fn parse_prop(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut index = *i;
    match &stream[index] {
        stream::Token::Predicate(pred) => {
            *i += 1;
            if index + 1 < stream.0.len() {
                match &stream[index + 1] {
                    stream::Token::Bracket(stream::Bracket::Close) => {
                        *i += 1;
                        return Ok(Proposition::Predicate(pred.clone()));
                    }
                    stream::Token::Operator(op) => {
                        *i += 1;
                        return match_op(
                            op,
                            i,
                            stream,
                            Proposition::Predicate(pred.clone()),
                            parse_prop,
                        );
                    }
                    _ => Err(ParseError),
                }
            } else {
                return Ok(Proposition::Predicate(pred.clone()));
            }
        }
        stream::Token::Bracket(stream::Bracket::Open) => {
            *i += 1;
            let mut prop = parse_prop(i, stream)?;
            index = *i;

            if index + 1 < stream.len() {
                prop = match_op_prop(*i, i, stream, prop)?;
            }

            Ok(prop)
        }
        stream::Token::Operator(stream::Operator::Not) => {
            *i += 1;
            handle_not(i, stream)
        }
        _ => Err(ParseError),
    }
}

pub fn handle_not(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut index = *i;
    if index < stream.len() {
        let prop = match &stream[index] {
            stream::Token::Predicate(pred) => {
                *i += 1;
                Ok(Proposition::Composition(Box::new(Operator::Not(
                    Proposition::Predicate(pred.clone()),
                ))))
            }
            stream::Token::Bracket(stream::Bracket::Open) => {
                *i += 1;
                Ok(Proposition::Composition(Box::new(Operator::Not(
                    parse_prop(i, stream)?,
                ))))
            }
            _ => Err(ParseError),
        }?;
        index = *i;

        if index + 1 < stream.len() {
            match_op_prop(*i, i, stream, prop)
        } else {
            return Ok(prop);
        }
    } else {
        Err(ParseError)
    }
}

pub fn match_op_prop(
    index: usize,
    i: &mut usize,
    stream: &stream::TokenStream,
    prop: Proposition,
) -> Result<Proposition, ParseError> {
    match &stream[index] {
        stream::Token::Bracket(stream::Bracket::Close) => {
            *i += 1;

            return Ok(prop);
        }
        stream::Token::Operator(op) => {
            *i += 1;
            match_op(op, i, stream, prop, parse_prop)
        }
        _ => Err(ParseError),
    }
}

pub fn match_op(
    op: &stream::Operator,
    i: &mut usize,
    stream: &stream::TokenStream,
    prop: Proposition,
    parser: impl Fn(&mut usize, &stream::TokenStream) -> Result<Proposition, ParseError>,
) -> Result<Proposition, ParseError> {
    match op {
        stream::Operator::And => Ok(Proposition::Composition(Box::new(Operator::And(
            prop,
            parser(i, stream)?,
        )))),
        stream::Operator::Or => Ok(Proposition::Composition(Box::new(Operator::Or(
            prop,
            parser(i, stream)?,
        )))),
        stream::Operator::Implies => Ok(Proposition::Composition(Box::new(Operator::Implies(
            prop,
            parser(i, stream)?,
        )))),
        stream::Operator::Not => Err(ParseError),
    }
}

impl std::fmt::Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = 90 + (rand::random::<u32>() % 7);
        match self {
            Proposition::Predicate(pred) => write!(f, "{}", pred),
            Proposition::Condition(cond) => match cond {
                Condition::True => write!(f, "T"),
                Condition::False => write!(f, "F"),
            },
            Proposition::Composition(comp) => match comp.as_ref() {
                Operator::And(a, b) => {
                    write!(
                        f,
                        "\x1b[{}m(\x1b[0m{} \x1b[{}m\\land\x1b[0m {}\x1b[{}m)\x1b[0m",
                        color, a, color, b, color
                    )
                }
                Operator::Or(a, b) => write!(
                    f,
                    "\x1b[{}m(\x1b[0m{} \x1b[{}m\\lor\x1b[0m {}\x1b[{}m)\x1b[0m",
                    color, a, color, b, color
                ),
                Operator::Implies(a, b) => write!(f, "({} \\implies {})", a, b),
                Operator::Not(a) => write!(f, "\\neg {}", a),
            },
        }
    }
}

impl Into<String> for Proposition {
    fn into(self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod test_operators {
    use super::*;
    use stream;

    #[test]
    fn test_partial_eq() {
        let cases = vec![(
            Proposition::new_or("A", Proposition::new_and("C", "B")),
            Proposition::new_or(Proposition::new_and("B", "C"), "A"),
        )];

        cases
            .into_iter()
            .for_each(|(input, expect)| assert_eq!(expect, input));
    }

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                stream::TokenStream(vec![
                    stream::Token::Predicate("A".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Predicate("B".to_string()),
                ]),
                Proposition::new_or("A", "B"),
            ),
            (
                stream::TokenStream(vec![
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Predicate("A".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Predicate("B".to_string()),
                    stream::Token::Bracket(stream::Bracket::Close),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Predicate("C".to_string()),
                ]),
                Proposition::new_and(Proposition::new_not(Proposition::new_or("A", "B")), "C"),
            ),
            (
                stream::TokenStream(vec![
                    stream::Token::Predicate("a".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    //
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Predicate("c".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Operator(stream::Operator::Not),
                    //
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Predicate("a".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    //
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Predicate("b".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Predicate("e".to_string()),
                    stream::Token::Bracket(stream::Bracket::Close),
                    //
                    stream::Token::Bracket(stream::Bracket::Close),
                    //
                    stream::Token::Operator(stream::Operator::And),
                    //
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Predicate("d".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    //
                    stream::Token::Bracket(stream::Bracket::Open),
                    stream::Token::Predicate("b".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Predicate("c".to_string()),
                    stream::Token::Bracket(stream::Bracket::Close),
                    //
                    stream::Token::Bracket(stream::Bracket::Close),
                    //
                    stream::Token::Bracket(stream::Bracket::Close),
                    //
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Predicate("_f".to_string()),
                ]),
                Proposition::new_and(Proposition::new_not(Proposition::new_or("A", "B")), "C"),
            ),
            (
                stream::TokenStream(vec![
                    stream::Token::Predicate("a".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Predicate("c".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Predicate("a".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Predicate("b".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Predicate("d".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Predicate("b".to_string()),
                    stream::Token::Operator(stream::Operator::And),
                    stream::Token::Predicate("c".to_string()),
                    stream::Token::Operator(stream::Operator::Or),
                    stream::Token::Operator(stream::Operator::Not),
                    stream::Token::Predicate("_f".to_string()),
                ]),
                Proposition::new_and(Proposition::new_not(Proposition::new_or("A", "B")), "C"),
            ),
        ];

        cases.into_iter().for_each(|(input, expect)| {
            let parsed = TryInto::<Proposition>::try_into(input).unwrap();
            // println!("{:?}", parsed);
            println!("{}", parsed);
            // assert_eq!(expect, parsed);
        });
    }
}
