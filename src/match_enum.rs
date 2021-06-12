#![allow(unused)]

fn plus(x: Option<i8>) -> Option<i8> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}

fn main() {
    let a: Option<i8> = Some(5);
    let b: Option<i8> = None;
    println!("{:?}, {:?}", plus(a), plus(b));
}
