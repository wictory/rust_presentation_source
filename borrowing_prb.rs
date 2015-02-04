
#[derive(Show)]
struct Type {
    val: i64
}

fn add(a: Type, b: Type) -> Type {
    Type { val: a.val + b.val }
}

fn main() {
    let a = Type { val: 1 };
    let b = Type { val: 2 };
    let c = add(a, b);
    println!("{:?} + {:?} = {:?}", a, b, c);
}
