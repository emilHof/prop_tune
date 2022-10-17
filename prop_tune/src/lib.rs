pub use prop_tune_core::{operators, stream};
pub use prop_tune_macro::simplify;

fn test_belt() {
let cases = vec![true, false];

    for &a in cases.iter() {
        for &b in cases.iter() {
            for &c in cases.iter() {
                for &d in cases.iter() {
                    for &e in cases.iter() {
                        /*
                        assert_eq!(
                             ((c && !(d || !e) || (a && !d))),
                             simplify!(
                                ((c && !(d || !e) || (a && !d)))
                            )
                        );
                        assert_eq!(
                            ((a || !(b && !a) && (c || !d)) && ((c && (a && !b) || (d && e)))),
                             simplify!(
                                 (a || !(b && !a) && (c || !d)) && ((c && (a && !b) || (d && e)))
                            )
                        );
                        assert_eq!(
                            ((a || !(b && !a) && (c || !d)) /*&& ((c && (a && !b) || (d && e))) */),
                             simplify!(
                            ((a || !(b && !a) && (c || !d)) /*&& ((c && (a && !b) || (d && e))) */)
                            )
                        );
                        assert_eq!(
                            (c && (a && !b)),
                             simplify!(
                            ((c && (a && !b)))
                            )
                        );
                        assert_eq!(
                            ((d && e)),
                             simplify!(
                            ((d && e))
                            )
                        );
                        */
                        // case 1
                        assert_eq!(
                            a && (b || !c),
                            simplify!(a && (b || !c))
                        );

                        // case 2
                        assert_eq!(
                            (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c))),
                            simplify!(
                                (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))
                            )
                        );
                        
                        // case 3
                        assert_eq!(
                            (((a || (!(b && !a) && (c || !d))) && (!(((c && ((a && !b) || (d && e))))))) 
                                 && ((c && (!(d || !e) || (a && !d))))),
                             simplify!(
                                 (((a || (!(b && !a) && (c || !d))) && (!(((c && ((a && !b) || (d && e))))))) 
                                     && ((c && (!(d || !e) || (a && !d)))))
                            )
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_lib {
    use super::*;

    fn eval(a: bool, b: bool, c: bool, d: bool) -> bool {
        if simplify!{c && !(a && (!c && d)) && b} {
            true
        } else {
            false
        }
    }

    fn t1() {
        let cases = vec![true, false];

        for &a in cases.iter() {
            for &b in cases.iter() {
                for &c in cases.iter() {
                    for &d in cases.iter() {
                        for &e in cases.iter() {
                            if simplify!(
                                ((e || !(a && !(b && c))) && !(c && a)) 
                                && ((a && (b || !c)) && !((d || !b) && !(a && c)))
                            ) { true } else { false };
                        }
                    }
                }
            }
        }
    }

    fn t2() {
        let cases = vec![true, false];
        
        for &a in cases.iter() {
            for &b in cases.iter() {
                for &c in cases.iter() {
                    for &d in cases.iter() {
                        for &e in cases.iter() {
                            if 
                            ((e || !(a && !(b && c))) && !(c && a)) 
                            && ((a && (b || !c)) && !((d || !b) && !(a && c)))
                            { true } else { false };
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_macro() {
        println!("{}", eval(true, true, false, false))
    }

    #[test]
    fn test_equiv() {
        test_belt()
    }

    #[test]
    fn test_t1() {
        use std::time::Instant;
        
        let now = Instant::now();

        (0..100_000).for_each(|_| t1());

        println!("t1: {:.2?}", now.elapsed());
    }

    #[test]
    fn test_t2() {
        use std::time::Instant;
        
        let now = Instant::now();

        (0..100_000).for_each(|_| t2());

        println!("t2: {:.2?}", now.elapsed());
    }
}
