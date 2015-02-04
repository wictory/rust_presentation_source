
use CantorNumber::{Zero, One, Two, Three, Infinity};

#[derive(Show)]
enum CantorNumber {
    Zero,
    One,
    Two,
    Three,
    Infinity
}

fn add(a: &CantorNumber, b: &CantorNumber) -> CantorNumber {
    let an = to_numer(a);
    let bn = to_numer(b);
    to_cantor(match (an, bn) {
        (Some(ai), Some(bi)) => Some(ai + bi),
        _                    => None
    })
}

fn to_numer(a: &CantorNumber) -> Option<u8> {
    match *a {
        Zero     => Some(0),
        One      => Some(1),
        Two      => Some(2),
        Three    => Some(3),
        Infinity => None
    }
}

fn to_cantor(a: Option<u8>) -> CantorNumber {
    match a {
        Some(0)        => Zero,
        Some(1)        => One,
        Some(2)        => Two,
        Some(3)        => Three,
        None | Some(_) => Infinity
    }
}

fn main() {
    let a = One;
    let b = One;
    println!("{:?} + {:?} = {:?}", a, b, add(&a, &b));
}
