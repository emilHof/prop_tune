pub use prop_tune_core::{operators, stream};
pub use prop_tune_macro::simplify;

fn eval(a: bool, b: bool, c: bool, d: bool) -> bool {
    if simplify!{c && !(a && (!c && d)) && b} {
        true
    } else {
        false
    }
}

fn test_belt() {
    let cases = vec![true, false];

    for &a in cases.iter() {
        for &b in cases.iter() {
            for &c in cases.iter() {
                for &d in cases.iter() {
                    for &e in cases.iter() {
                        assert_eq!(
                            (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c))),
                            simplify!(
                                (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))
                            )
                        )
                    }
                }
            }
        }
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

#[cfg(test)]
mod test_lib {
    use super::*;

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

        (0..100_000_0).for_each(|_| t1());

        println!("{:.2?}", now.elapsed());
    }

    #[test]
    fn test_t2() {
        use std::time::Instant;
        
        let now = Instant::now();

        (0..100_000_0).for_each(|_| t2());

        println!("{:.2?}", now.elapsed());
    }
}
