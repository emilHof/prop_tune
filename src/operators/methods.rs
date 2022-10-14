use std::ops::Index;
use std::cmp::Ordering;

use super::*;

impl Proposition {
    pub fn demorg(self) -> Proposition {
        match self {
            Proposition::Predicate(pred) => Proposition::Predicate(pred), 
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => Proposition::new_and(a.demorg(), b.demorg()), 
                Operator::Or(a, b) => Proposition::new_or(a.demorg(), b.demorg()), 
                Operator::Implies(a, b) => Proposition::new_or(a, Proposition::new_not(b)).demorg(),
                    Operator::Not(a) => match a {
                    Proposition::Predicate(pred) => Proposition::new_not(pred),
                        Proposition::Condition(cond) => Proposition::Condition(match cond {
                        Condition::True => Condition::False,
                        Condition::False => Condition::True,
                    }),
                    Proposition::Composition(comp) => match *comp {
                        Operator::And(a, b) => Proposition::new_or(
                            Proposition::new_not(a).demorg(),
                            Proposition::new_not(b).demorg()
                        ),
                        Operator::Or(a, b) => Proposition::new_and(
                            Proposition::new_not(a).demorg(),
                            Proposition::new_not(b).demorg()
                        ),
                        Operator::Implies(a, b) => Proposition::new_and(Proposition::new_not(a), b).demorg(),
                        Operator::Not(a) => a.demorg(),
                    }
                }
            },
            Proposition::Condition(cond) => Proposition::Condition(cond),
        }
    }

    pub fn normal(self) -> Proposition {
        match self.demorg() {
            Proposition::Predicate(pred) => pred.into(),
            Proposition::Condition(cond) => Proposition::Condition(cond),
            Proposition::Composition(comp) => match *comp {
                Operator::And(a, b) => {
                    match a.normal() {
                        Proposition::Predicate(a) => {
                            let a = Proposition::Predicate(a);
                            match b.normal() {
                                Proposition::Predicate(b) => Proposition::new_and(a, b),
                                Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                Proposition::Composition(comp) => match *comp {
                                    Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c).normal()),
                                    Operator::Or(b, c) => Proposition::new_or(Proposition::new_and(a.clone(), b).normal(), Proposition::new_and(a, c).normal()),
                                    Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                    Operator::Implies(_, _) => unreachable!(),
                                }
                            }
                        },
                        Proposition::Condition(a) => {
                            let a = Proposition::Condition(a);
                            match b.normal() {
                                Proposition::Predicate(b) => Proposition::new_and(a, Proposition::Predicate(b)),
                                Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c)),
                                    Operator::Or(b, c) => Proposition::new_or(Proposition::new_and(a.clone(), b), Proposition::new_and(a, c)),
                                    Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                    Operator::Implies(_, _) => unreachable!(),
                                },
                            }
                        },
                        Proposition::Composition(a) => match *a {
                            Operator::And(a, c) => {
                                let a = Proposition::new_and(a, c);
                                match b.normal() {
                                    Proposition::Predicate(b) => Proposition::new_and(a, Proposition::Predicate(b)),
                                    Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c)),
                                        Operator::Or(b, c) => Proposition::new_or(
                                            Proposition::new_and(a.clone(), b), 
                                            Proposition::new_and(a, c)
                                        ),
                                        Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                }
                            },
                            Operator::Or(a, c) => {
                                match b.normal() {
                                    Proposition::Predicate(b) => Proposition::new_or(
                                        Proposition::new_and(a, b.clone()), 
                                        Proposition::new_and(c, Proposition::Predicate(b.clone()))
                                    ),
                                    Proposition::Condition(cond) => {
                                        let b = Proposition::Condition(cond);
                                        Proposition::new_or(Proposition::new_and(a, b.clone()), Proposition::new_and(c, b))
                                    },
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, d) => {
                                            let b = Proposition::new_and(b, d).normal();
                                            let a = Proposition::new_and(a, b.clone()).normal();
                                            let c = Proposition::new_and(c, b.clone()).normal();

                                            Proposition::new_or(a, c) 
                                        },
                                        Operator::Or(b, d) => {
                                            let ab = Proposition::new_and(a.clone(), b.clone()).normal();
                                            let ad = Proposition::new_and(a.clone(), d.clone()).normal();
                                            let cb = Proposition::new_and(c.clone(), b.clone()).normal();
                                            let cd = Proposition::new_and(c.clone(), d.clone()).normal();

                                            Proposition::new_or(ab, Proposition::new_or(ad, Proposition::new_or(cb, cd)))
                                        },
                                        Operator::Not(b) => {
                                            let b = Proposition::new_not(b);

                                            Proposition::new_or(Proposition::new_and(a, b.clone()), Proposition::new_and(c, b))
                                        },
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                }.normal()
                            },
                            Operator::Not(a) => {
                                let a = Proposition::new_not(a);
                                match b.normal() {
                                    Proposition::Predicate(b) => Proposition::new_and(a, b),
                                    Proposition::Condition(cond) => Proposition::new_and(a, Proposition::Condition(cond)),
                                    Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c)),
                                        Operator::Or(b, c) => Proposition::new_or(
                                            Proposition::new_and(a.clone(), b), 
                                            Proposition::new_and(a, c)
                                        ),
                                        Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                        Operator::Implies(_, _) => unreachable!(),
                                    }
                                }
                            },
                            Operator::Implies(_, _) => unreachable!(),
                        },
                    }
                },
                Operator::Or(a, b) => Proposition::new_or(a.normal(), b.normal()),
                Operator::Implies(a, b) => Proposition::new_or(a.normal(),Proposition::new_not(b.normal())),
                Operator::Not(a) => Proposition::new_not(a.normal()),
            }
        }
    }

    pub fn simplify(self) -> Proposition {
        match self.normal() {
            Proposition::Predicate(pred) => Proposition::Predicate(pred),
            Proposition::Condition(cond) => Proposition::Condition(cond),
            Proposition::Composition(comp) => match *comp {
                Operator::Not(a) => Proposition::new_not(a),
                Operator::And(a, b) => {
                    let (mut preds, mut nots) = (std::collections::HashSet::new(), std::collections::HashSet::new());
                    match a.account_predicates(&mut preds, &mut nots) && b.account_predicates(&mut preds, &mut nots) {
                        true => {
                            match preds.len().cmp(&1) {
                                Ordering::Less => Self::construct_ands(nots.into_iter().collect()),
                                _ => match nots.len().cmp(&1) {
                                    Ordering::Less => Self::construct_ands(preds.into_iter().collect()),
                                    _ => Proposition::new_and(Self::construct_ands(preds.into_iter().collect()), Self::construct_ands(nots.into_iter().collect()))
                                }
                            }
                        },
                        false => Proposition::Condition(Condition::False),
                    }
                },
                Operator::Or(a, b) => Proposition::new_or(a.simplify(), b.simplify()),
                Operator::Implies(_, _) => unreachable!(),
            }
        }
    }

    fn construct_ands(predicates: Vec<String>) -> Proposition {
        let n = predicates.len() - 1;
        match n.cmp(&1) {
            Ordering::Less => Proposition::Predicate(predicates.get(0).expect("there to be more predicates").clone()),
            _ => {
                predicates.iter()
                    .take(n).
                    fold(
                        Proposition::new_pred(predicates.index(n)), 
                        |prop, pred| Proposition::new_and(pred.clone(), prop)
                    )
            }
        }
    }

    fn account_predicates(&self, preds: &mut std::collections::HashSet<String>, nots: &mut std::collections::HashSet<String>) -> bool {
        match self {
            Proposition::Condition(cond) => match cond {
                Condition::True => true,
                Condition::False => false,
            },
            Proposition::Predicate(pred) => {
                preds.insert(pred.clone());
                !nots.contains(pred) 
            },
            Proposition::Composition(comp) => match comp.as_ref() {
                Operator::Not(prop) => match prop {
                    Proposition::Predicate(pred) => {
                        nots.insert(pred.clone());
                        !preds.contains(pred)
                    },
                    _ => unreachable!(),
                },
                Operator::And(a, b) => a.account_predicates(preds, nots) && b.account_predicates(preds, nots),
                _ => unreachable!(),
            }
        }
    } 
}

#[cfg(test)]
mod test_procs {
    use super::*;

    #[test]
    fn test_demorg() {
        let input = vec![
            Proposition::new_not(Proposition::new_or("A", "A")),
        ];
        let expected = vec![
            Proposition::new_and(Proposition::new_not("A"),Proposition::new_not("A"))
        ];

        input.into_iter().zip(expected.into_iter()).for_each( |(actual, expected)| {
                let actual = actual.demorg();
                println!("{:?}, {:?}", expected, actual);
                assert_eq!(expected, actual)
            }
        );
    }

    #[test]
    fn test_normal() {
        let cases = vec![
            (
                Proposition::new_and("A", Proposition::new_or("B", Proposition::new_not("C"))),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_and("A", Proposition::new_not("C")))
            ),
            (
                Proposition::new_and("A", Proposition::new_or("B", Proposition::new_not(Proposition::new_and("C", "D")))),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_or(
                    Proposition::new_and(
                        "A", 
                        Proposition::new_not("C")
                    ), 
                    Proposition::new_and("A", Proposition::new_not("D"))
                )),
            ),
            (
                Proposition::new_and(Proposition::new_or("A", "C"), Proposition::new_or("B", "D")),
                Proposition::new_or(Proposition::new_and("A", "B"), Proposition::new_or(
                    Proposition::new_and("A", "D"), 
                    Proposition::new_or(Proposition::new_and("C", "B"), Proposition::new_and("C", "D"))
                ))
            ),
            (
                // ((A \lor B) \land C) \land (D \lor E)
                // (( A \land C ) \lor (B \land C)) \land (D \lor E)
                // (A \land C \land D) \lor (A \land C \land E) \lor (B \land C \land D) \lor (B \land C \land E)
                Proposition::new_and(Proposition::new_and(Proposition::new_or("A", "B"), "C"), Proposition::new_or("D", "E")),
                Proposition::new_or(
                    Proposition::new_and(Proposition::new_and("A", "C"), "D"), 
                    Proposition::new_or(
                        Proposition::new_and(Proposition::new_and("A", "C"), "E"), 
                        Proposition::new_or(
                            Proposition::new_and(Proposition::new_and("B", "C"), "D"), 
                            Proposition::new_and(Proposition::new_and("B", "C"), "E")
                        )
                    )
                )
            ),
            (
                // \neg ((A \lor B) \implies ((C \land D) \lor E))
                // (\neg A \land \neg B) \land ((C \land D) \lor E)
                // (\neg A \land \neg B \land C \land D) \lor (\neg A \land \neg B \land E)
                Proposition::new_not(Proposition::new_implies(Proposition::new_or("A", "B"), Proposition::new_or(Proposition::new_and("C", "D"), "E"))),
                Proposition::new_or(
                    Proposition::new_and(Proposition::new_and(Proposition::new_not("A"), Proposition::new_not("B")), Proposition::new_and("C", "D")), 
                    Proposition::new_and(Proposition::new_and(Proposition::new_not("A"), Proposition::new_not("B")), "E") 
                )
            )
        ];

        cases.into_iter().for_each(|(input, expected)| {
            let actual = input.normal();
            println!("{}", &actual);
            assert_eq!(expected, actual)
        })
    }

    #[test]
    fn test_simplify() {
        let cases = vec![
            (
                Proposition::new_and(Proposition::new_and("A", "C"), "C"),
                Proposition::new_and("A", "C"),
            ),
            (
                
                Proposition::new_and("A", Proposition::new_not("A")),
                Proposition::Condition(Condition::False)
            )
        ];

        cases.into_iter().for_each(|(input, expected)| assert_eq!(expected, input.simplify()));
    }
}
