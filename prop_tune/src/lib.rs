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
                        assert_eq!(a && (b || !c), simplify!(a && (b || !c)));

                        // case 2
                        assert_eq!(
                            (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c))),
                            simplify!(
                                (e && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))
                            )
                        );

                        // case 3
                        assert_eq!(
                            (((a || (!(b && !a) && (c || !d)))
                                && (!(c && ((a && !b) || (d && e)))))
                                && (c && (!(d || !e) || (a && !d)))),
                            simplify!(
                                (((a || (!(b && !a) && (c || !d)))
                                    && (!(c && ((a && !b) || (d && e)))))
                                    && (c && (!(d || !e) || (a && !d))))
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
    ((e || !(a && !(b && c))) && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c))) 
}

#[allow(dead_code)]
fn ft1s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(((e || !(a && !(b && c))) && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))) 
}

#[allow(dead_code)]
fn ft2p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    ((a && !(b && !(d || !c))) && ((!a || _f) && (c && !(b || d)))) && (a || !(e && !c))
}

#[allow(dead_code)]
fn ft2s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(((a && !(b && !(d || !c))) && ((!a || _f) && (c && !(b || d)))) && (a || !(e && !c)))
}

#[allow(dead_code)]
fn ft3p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    ((b || !( a || b )) && (!c || ( !(a || e) && (_f && !(e && d)))) 
     || ((c && !(d || !a)) && !(d || !e))) && ((!e && (d || b)) && (a || !_f))
}

#[allow(dead_code)]
fn ft3s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(
        ((b || !( a || b )) && (!c || ( !(a || e) && (_f && !(e && d)))) 
         || ((c && !(d || !a)) && !(d || !e))) && ((!e && (d || b)) && (a || !_f))
    )
}

#[allow(dead_code)]
const TF: [bool; 2] = [true, false];

#[cfg(test)]
mod test_lib {
    use super::*;

    fn eval(a: bool, b: bool, c: bool, d: bool) -> bool {
        if simplify! {c && !(a && (!c && d)) && b} {
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
        test_belt();
        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| 
                TF.iter().for_each(|&b| 
                    TF.iter().for_each(|&c| 
                        TF.iter().for_each(|&d| 
                            TF.iter().for_each(|&e| 
                                TF.iter().for_each(|&f| {
                                assert_eq!(
                                    ft2s(a, b, c, d, e, f),
                                        ft2p(a, b, c, d, e, f)
                                    );
                                    assert_eq!(
                                        ft3s(a, b, c, d, e, f),
                                        ft3p(a, b, c, d, e, f)
                                    )
                                })
                            )
                        )
                    )
                )
            )
        });
    }

    #[test]
    fn test_t1() {
        use std::time::Instant;

        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| 
                TF.iter().for_each(|&b| 
                    TF.iter().for_each(|&c| 
                        TF.iter().for_each(|&d| 
                            TF.iter().for_each(|&e| 
                                TF.iter().for_each(|&f| {
                                    ft1s(a, b, c, d, e, f);
                                    ft2s(a, b, c, d, e, f);
                                    ft3s(a, b, c, d, e, f);
                                })
                            )
                        )
                    )
                )
            )
        });

        println!("t1: {:.2?}", now.elapsed());
    }

    #[test]
    fn test_t2() {
        use std::time::Instant;

        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| 
                TF.iter().for_each(|&b| 
                    TF.iter().for_each(|&c| 
                        TF.iter().for_each(|&d| 
                            TF.iter().for_each(|&e| 
                                TF.iter().for_each(|&f| {
                                    ft1p(a, b, c, d, e, f);
                                    ft2p(a, b, c, d, e, f);
                                    ft3p(a, b, c, d, e, f);
                                })
                            )
                        )
                    )
                )
            )
        });

        println!("t2: {:.2?}", now.elapsed());
    }
}
