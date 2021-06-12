#![allow(unused)]

use std::io;
use io::Write;

fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}

fn input(prefix: &str) -> String {
    print!("{}", prefix);
    io::stdout().flush();

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line from stdin");

    return trim_newline(&mut inp);
}

fn gen_fib(n: i32) -> Vec<u128> {
    // Create i32 vector starting with 1, 1
    let mut vec: Vec<u128> = [1, 1].to_vec();

    // For each until n
    for _ in 0..n {
        vec.push(
            vec[vec.len() - 1] + vec[vec.len() - 2]
        );
        vec = vec[(vec.len() - 2)..];
    }

    return vec;
}

fn main() {
    let mut n = 5;
    match input("Fib n: ").parse::<i32>() {
        Ok(val) => {n = val;},
        Err(err) => {
            println!("Invalid number, defaulting to 5");
        }
    }

    let fib = gen_fib(n);
    println!("{}th term: {}", n, fib[fib.len() - 1]);
}
