#[derive(Debug, PartialEq)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[test]
fn test_tier() {
    let s = Tier::S;
    let a = Tier::A;
    let b = Tier::B;
    let c = Tier::C;
    let d = Tier::D;
    let e = Tier::E;
    let f = Tier::F;

    assert_eq!(s, Tier::S);
    assert_eq!(a, Tier::A);
    assert_eq!(b, Tier::B);
    assert_eq!(c, Tier::C);
    assert_eq!(d, Tier::D);
    assert_eq!(e, Tier::E);
    assert_eq!(f, Tier::F);
}
