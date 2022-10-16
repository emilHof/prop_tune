use super::*;

impl Proposition {
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
                                    Operator::Or(b, c) => Proposition::new_or(
                                        Proposition::new_and(a.clone(), b).normal(), 
                                        Proposition::new_and(a, c).normal()
                                    ),
                                    Operator::Not(b) => Proposition::new_and(a, Proposition::new_not(b)),
                                    Operator::Implies(_, _) => unreachable!(),
                                }
                            }
                        },
                        Proposition::Condition(a) => {
                            let a = Proposition::Condition(a);
                            match b.normal() {
                                Proposition::Predicate(b) => Proposition::new_and(
                                    a, 
                                    Proposition::Predicate(b)
                                ),
                                Proposition::Condition(cond) => Proposition::new_and(
                                    a, 
                                    Proposition::Condition(cond)
                                ),
                                Proposition::Composition(comp) => match *comp {
                                        Operator::And(b, c) => Proposition::new_and(a, Proposition::new_and(b, c)),
                                    Operator::Or(b, c) => Proposition::new_or(
                                        Proposition::new_and(a.clone(), b), 
                                        Proposition::new_and(a, c)
                                    ),
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
}
