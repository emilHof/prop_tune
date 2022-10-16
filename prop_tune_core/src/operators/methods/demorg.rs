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
}
