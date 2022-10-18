pub use prop_tune_core::{operators, stream};
pub use prop_tune_macro::simplify;

#[allow(dead_code)]
fn test_belt() {
let cases = vec![true, false];
    for &a in cases.iter() {
        for &b in cases.iter() {
            for &c in cases.iter() {
                for &d in cases.iter() {
                    for &e in cases.iter() {
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

#[allow(dead_code)]
fn ft1p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    if ((e || !(a && !(b && c))) && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c))) { 
        true 
    } else { 
        false 
    }
}

#[allow(dead_code)]
fn ft1s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    if simplify!(((e || !(a && !(b && c))) && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))) { 
        true 
    } else { 
        false 
    }
}

#[allow(dead_code)]
const TF: [bool; 2] = [true, false];

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
        
        let fns = vec![ft1s];

        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| 
                TF.iter().for_each(|&b| 
                    TF.iter().for_each(|&c| 
                        TF.iter().for_each(|&d| 
                            TF.iter().for_each(|&e| 
                                TF.iter().for_each(|&f| 
                                    fns.iter().for_each(|func| { func(a, b, c, d, e, f); })))))))
            
        });

        println!("t1: {:.2?}", now.elapsed());
    }

    #[test]
    fn test_t2() {
        use std::time::Instant;

        let fns = vec![ft1p];
        
        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| 
                TF.iter().for_each(|&b| 
                    TF.iter().for_each(|&c| 
                        TF.iter().for_each(|&d| 
                            TF.iter().for_each(|&e| 
                                TF.iter().for_each(|&f| 
                                    fns.iter().for_each(|func| { func(a, b, c, d, e, f); })))))))
            
        });

        println!("t2: {:.2?}", now.elapsed());
    }
}
