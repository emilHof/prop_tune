mod demorg;
mod normal;
mod simplify;

use std::ops::Index;
use std::cmp::Ordering;
use super::*;

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
            ),
            (
                Proposition::new_or(
                    Proposition::new_or(
                        Proposition::new_and(
                            "A", 
                            Proposition::new_not("A")
                        ), Proposition::new_and(
                        "A", 
                        Proposition::new_not("A"))
                    ), 
                    Proposition::new_and("A", "D")) ,
                Proposition::new_and("A", "D")
            ),
            (
                Proposition::new_and("c", Proposition::new_not(Proposition::new_and("a", "b"))),
                Proposition::new_or(
                    Proposition::new_and("c", Proposition::new_not("a")), 
                    Proposition::new_and("c", Proposition::new_not("b"))
                ),
            )
        ];

        cases.into_iter().for_each(|(input, expected)| assert_eq!(expected, input.simplify()));
    }
}
