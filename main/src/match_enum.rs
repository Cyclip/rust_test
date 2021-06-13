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

    let val = Some(3u8);

    // Match
    match val {
        Some(3) => println!("Match: is three"),
        _ => println!("Match: is not three"),
    }

    // if let pattern = val {expression/statement}
    if let Some(3) = val {
        println!("If let: is three");
    } else {
        println!("If let: is not three");
    }
}
