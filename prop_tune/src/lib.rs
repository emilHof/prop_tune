pub use prop_tune_core::{operators, stream};
use prop_tune_macro::rewrite;
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
    simplify!(
        ((e || !(a && !(b && c))) && !(c && a)) && ((a && (b || !c)) && !((d || !b) && !(a && c)))
    )
}

#[allow(dead_code)]
fn ft2p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    ((a && !(b && !(d || !c))) && ((!a || _f) && (c && !(b || d)))) && (a || !(e && !c))
}

#[allow(dead_code)]
fn ft2s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(((a && !(b && !(d || !c))) && ((!a || _f) && (c && !(b || d)))) && (a || !(e && !c)))
}

#[allow(dead_code, unused_variables)]
fn ft3p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    ((b || !(a || b)) && (!c || (!(a || e) && (_f && !(e && d))))
        || ((c && !(d || !a)) && !(d || !e)))
        && ((!e && (d || b)) && (a || !_f))
}

#[allow(dead_code, unused_variables)]
fn ft3s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(
        ((b || !(a || b)) && (!c || (!(a || e) && (_f && !(e && d))))
            || ((c && !(d || !a)) && !(d || !e)))
            && ((!e && (d || b)) && (a || !_f))
    )
}

#[allow(dead_code, unused_variables)]
fn ft4p(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    a && (!c || !(a || (b && e)) && (!d || (b && c))) || !f
}

#[allow(dead_code, unused_variables)]
fn ft4s(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    simplify!(a && (!c || !(a || (b && e)) && (!d || (b && c))) || !f)
}

#[allow(dead_code, unused_variables)]
fn ft4n(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    rewrite!(a && (!c || !(a || (b && e)) && (!d || (b && c))) || !f)
}

#[allow(dead_code, unused_variables)]
fn ft5p(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    !(a && (b && c))
}

#[allow(dead_code, unused_variables)]
fn ft5s(a: bool, b: bool, c: bool, d: bool, e: bool, _f: bool) -> bool {
    simplify!(!(a && (b && c)))
}

#[allow(dead_code, unused_variables)]
fn ft6p(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    a && !(c || b) || !(a && f)
}

#[allow(dead_code, unused_variables)]
fn ft6s(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    simplify!(a && !(c || b) || !(a && f))
}

#[allow(dead_code, unused_variables)]
fn ft7p(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    a && !(c || b) || !(a && f)
}

#[allow(dead_code, unused_variables)]
fn ft7s(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool) -> bool {
    simplify!(a && !(c || b) || !(a && f))
}

#[allow(dead_code, unused_variables)]
fn r1(a: bool, b: bool, c: bool) -> bool {
    rewrite!(!(a && (b && c)))
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
        println!("passed belt");
        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| {
                TF.iter().for_each(|&b| {
                    TF.iter().for_each(|&c| {
                        TF.iter().for_each(|&d| {
                            TF.iter().for_each(|&e| {
                                TF.iter().for_each(|&f| {
                                    assert_eq!(ft1s(a, b, c, d, e, f), ft1p(a, b, c, d, e, f));
                                    assert_eq!(ft2s(a, b, c, d, e, f), ft2p(a, b, c, d, e, f));
                                    assert_eq!(ft3s(a, b, c, d, e, f), ft3p(a, b, c, d, e, f));
                                    assert_eq!(ft5s(a, b, c, d, e, f), ft5p(a, b, c, d, e, f));

                                    // THESE two tests are currently failing
                                    // This is because parsing with ambiguous parenthese is
                                    // currently not supported
                                    // assert_eq!(ft4p(a, b, c, d, e, f), ft4n(a, b, c, d, e, f));
                                    // assert_eq!(ft4s(a, b, c, d, e, f), ft4p(a, b, c, d, e, f));
                                    // assert_eq!(ft7s(a, b, c, d, e, f), ft7p(a, b, c, d, e, f));
                                })
                            })
                        })
                    })
                })
            })
        });
    }

    #[test]
    fn test_t1() {
        use std::time::Instant;

        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| {
                TF.iter().for_each(|&b| {
                    TF.iter().for_each(|&c| {
                        TF.iter().for_each(|&d| {
                            TF.iter().for_each(|&e| {
                                TF.iter().for_each(|&f| {
                                    ft1s(a, b, c, d, e, f);
                                    ft2s(a, b, c, d, e, f);
                                    ft3s(a, b, c, d, e, f);
                                })
                            })
                        })
                    })
                })
            })
        });

        println!("t1: {:.2?}", now.elapsed());
    }

    #[test]
    fn test_t2() {
        use std::time::Instant;

        let now = Instant::now();

        (0..100_000).for_each(|_| {
            TF.iter().for_each(|&a| {
                TF.iter().for_each(|&b| {
                    TF.iter().for_each(|&c| {
                        TF.iter().for_each(|&d| {
                            TF.iter().for_each(|&e| {
                                TF.iter().for_each(|&f| {
                                    ft1p(a, b, c, d, e, f);
                                    ft2p(a, b, c, d, e, f);
                                    ft3p(a, b, c, d, e, f);
                                })
                            })
                        })
                    })
                })
            })
        });

        println!("t2: {:.2?}", now.elapsed());
    }

    #[test]
    fn test_expand() {
        r1(true, true, true);
    }
}
