use super::*;

impl Proposition {
    pub fn simplify(self) -> Proposition {
        match self.normal() {
            Proposition::Predicate(pred) => Proposition::Predicate(pred),
            Proposition::Condition(cond) => Proposition::Condition(cond),
            Proposition::Composition(comp) => match *comp {
                Operator::Not(a) => Proposition::new_not(a),
                Operator::And(a, b) => {
                    let (mut preds, mut nots) = 
                        (std::collections::HashSet::new(), std::collections::HashSet::new());
                    match a.account_predicates(&mut preds, &mut nots) 
                        && b.account_predicates(&mut preds, &mut nots) {
                        true => {
                            match preds.len().cmp(&1) {
                                Ordering::Less => Self::chain_props(
                                    nots.into_iter().collect(), 
                                    |a| Proposition::new_not(a), 
                                    Proposition::new_and
                                ),
                                _ => match nots.len().cmp(&1) {
                                    Ordering::Less => Self::chain_props(
                                        preds.into_iter().collect(), 
                                        |a| a,
                                        Proposition::new_and
                                    ),
                                    _ => Proposition::new_and(
                                        Self::chain_props(
                                            preds.into_iter().collect(), 
                                            |a| a,
                                            Proposition::new_and
                                        ), 
                                        Self::chain_props(
                                            nots.into_iter().collect(), 
                                            |a| Proposition::new_not(a),
                                            |a, b| Proposition::new_and(a, b)
                                        )
                                    )
                                }
                            }
                        },
                        false => Proposition::Condition(Condition::False),
                    }
                },
                Operator::Or(a, b) => {
                    let mut props = std::collections::HashSet::new();
                    a.gather_propositions(&mut props);
                    b.gather_propositions(&mut props);

                    let props: Vec<Proposition> = props.into_iter()
                        .filter(|prop| prop.ne(&Proposition::Condition(Condition::False)))
                        .collect();

                    match props.len().cmp(&1) {
                        Ordering::Less => Proposition::new_false(),
                        _ => Self::chain_props(props, |a| a, Proposition::new_or)
                    }
                },
                Operator::Implies(_, _) => unreachable!(),
            }
        }
    }

    fn gather_propositions(self, props: &mut std::collections::HashSet<Proposition>) -> bool {
        match self {
            Proposition::Condition(Condition::True) => props.insert(Proposition::new_true()),
            Proposition::Predicate(pred) => props.insert(Proposition::new_pred(pred)),
            Proposition::Composition(cond) => match *cond {
                Operator::And(a, b) => props.insert(Proposition::new_and(a, b).simplify()),
                Operator::Or(a, b) => a.gather_propositions(props) && b.gather_propositions(props),
                _ => false,
            },
            _ => false,
        }
    }

    fn chain_props<P>(
        props: Vec<P>, 
        wrap: impl Fn(Proposition) -> Proposition, 
        comp: impl Fn(Proposition, Proposition) -> Proposition
    ) -> Proposition
        where P: Into<Proposition> + Clone
    {
        let n = props.len() - 1;
        match n.cmp(&1) {
            Ordering::Less => wrap(props.get(0).expect("there to be a proposition").clone().into()),
            _ => {
                props.iter()
                    .take(n)
                    .fold(
                        wrap(props.index(n).clone().into()),
                        |prev, prop| comp(prev, wrap(prop.clone().into()))
                    )
            }
        }
    }

    fn account_predicates(
        &self, 
        preds: &mut std::collections::HashSet<String>, 
        nots: &mut std::collections::HashSet<String>
    ) -> bool 
    {
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
