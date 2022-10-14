mod methods;
use crate::stream;

#[derive(Debug, Clone)]
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
            }
        }
    }
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

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, PartialEq, Clone)]
pub enum Proposition {
    Condition(Condition),
    Predicate(String),
    Composition(Box<Operator>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    True,
    False,
}

impl TryInto<Proposition> for crate::stream::TokenStream {
    type Error = ParseError;

    fn try_into(self) -> Result<Proposition, Self::Error> {
        if self.0.len() < 1 {
            return Err(ParseError)
        }
        
        let mut i = 0;
        
        parse_prop(&mut i, &self)
    }
}


fn parse_prop(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut first = *i;
    match &stream.0[first] {
        stream::Token::Predicate(pred) => {
            *i += 1;
            if first + 1 < stream.0.len() {
                match &stream.0[first + 1] {
                    stream::Token::Bracket(stream::Bracket::Close) => {
                        *i += 1;
                        return Ok(Proposition::Predicate(pred.clone()));
                    },
                    stream::Token::Operator(op) => {
                        *i += 1;
                        return match_op(op, i, stream, Proposition::Predicate(pred.clone()));
                    },
                    _ => Err(ParseError)
                }
            } else {
                return Ok(Proposition::Predicate(pred.clone()))
            }
        },
        stream::Token::Bracket(stream::Bracket::Open) => {
            *i += 1;
            let prop = parse_prop(i, stream)?;
            first = *i;

            if first + 1 < stream.0.len() {
                match_op_prop(*i, i, stream, prop)
            } else {
                return Ok(prop)
            }
        },
        stream::Token::Operator(stream::Operator::Not) => {
            *i += 1;
            handle_not(i, stream)
        },
        _ => Err(ParseError)
    } 
}

pub fn handle_not(i: &mut usize, stream: &stream::TokenStream) -> Result<Proposition, ParseError> {
    let mut first = *i;
    if first < stream.0.len() {

        let prop = match &stream.0[first] {
            stream::Token::Predicate(pred) => { *i += 1; Ok(Proposition::Composition(Box::new(Operator::Not(Proposition::Predicate(pred.clone()))))) },
            stream::Token::Bracket(stream::Bracket::Open) => {
                *i += 1;
                Ok(Proposition::Composition(Box::new(Operator::Not(parse_prop(i, stream)?))))
            },
            _ => Err(ParseError)
        }?;
        first = *i;

        if first + 1 < stream.0.len() {
            match_op_prop(*i, i, stream, prop)
        } else {
            return Ok(prop)
        }
    } else {
        Err(ParseError)
    }
}

pub fn match_op_prop(first: usize, i: &mut usize, stream: &stream::TokenStream, prop: Proposition) -> Result<Proposition, ParseError> {
    match &stream.0[first] {
        stream::Token::Bracket(stream::Bracket::Close) => {
            *i += 1;
            return Ok(prop);
        },
        stream::Token::Operator(op) => {
            *i += 1;
            match_op(op, i, stream, prop)
        },
        _ => Err(ParseError)
    }
}

pub fn match_op(op: &stream::Operator, i: &mut usize, stream: &stream::TokenStream, prop: Proposition) -> Result<Proposition, ParseError> { 
    match op {
        stream::Operator::And => {
            Ok(Proposition::Composition(Box::new(Operator::And(prop, parse_prop(i, stream)?))))
        },
        stream::Operator::Or => {
            Ok(Proposition::Composition(Box::new(Operator::Or(prop, parse_prop(i, stream)?))))
        },
        stream::Operator::Implies => {
            Ok(Proposition::Composition(Box::new(Operator::Implies(prop, parse_prop(i, stream)?))))
        },
        stream::Operator::Not => {
            Err(ParseError)
        }
    }
}

impl std::fmt::Display for Proposition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Proposition::Predicate(pred) => write!(f, "{}", pred),
            Proposition::Condition(cond) => match cond {
                Condition::True => write!(f, "T"),
                Condition::False => write!(f, "F"),
            },
            Proposition::Composition(comp) => match comp.as_ref() {
                Operator::And(a, b) => write!(f, "({} \\land {})", a, b),
                Operator::Or(a, b) => write!(f, "{} \\lor {}", a, b),
                Operator::Implies(a, b) => write!(f, "({} \\implies {})", a, b),
               Operator::Not(a) => write!(f, "\\neg {}", a),            
            }
        }
    }
}

impl Into<String> for Proposition {
    fn into(self) -> String {
        match self {
            Proposition::Predicate(pred) => format!("{}", pred),
            Proposition::Condition(cond) => match cond {
                Condition::True => format!("T"),
                Condition::False => format!("F"),
            },
            Proposition::Composition(comp) => match comp.as_ref() {
                Operator::And(a, b) => format!("{} \\land {}", a, b),
                Operator::Or(a, b) => format!("{} \\lor {}", a, b),
                Operator::Implies(a, b) => format!("{} \\implies {}", a, b),
               Operator::Not(a) => format!("\\neg {}", a),            
            }
        }
    }
}


#[cfg(test)]
mod test_operators {
    use super::*;

    #[test]
    fn test_composition() {
        let comp = Proposition::Composition(Box::new(Operator::And(Proposition::Predicate("A".to_string()), Proposition::Predicate("B".to_string()))));        
        println!("{:?}", comp);
    }

    #[test]
    fn test_complex_compostion() {
        let comp = Proposition::Composition(Box::new(Operator::And(
                    Proposition::Predicate("C".to_string()),
                    Proposition::Composition(Box::new(Operator::Or(
                                Proposition::Predicate("A".to_string()),
                                Proposition::Predicate("B".to_string())
                                )))
                    )));        
        println!("{:?}", comp);
        println!("{}", comp);
    }


    #[test]
    fn test_parsing() {
        use stream::Token;
        let stream = stream::TokenStream(vec![Token::Predicate("A".to_string()), Token::Operator(stream::Operator::And), Token::Predicate("B".to_string())]);
        let comp: Proposition = stream.try_into().ok().unwrap();
        println!("{:?}", comp)
    }

    #[test]
    fn test_partial_eq() {
        let cases = vec![
            (
                Proposition::new_or("A", Proposition::new_and("C", "B")),
                Proposition::new_or(Proposition::new_and("B", "C"), "A"),
            ),
        ];

        cases.into_iter().for_each(|(input, expect)| assert_eq!(expect, input));
    }
}
