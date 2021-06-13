#![allow(unused)]

use std::io;
use io::Write;

fn main() {
    let w = 5;
    let x = w;
    println!("{}", w);
    println!("{}", x);


    let y = String::from("hi");
    let z = y;
    println!("{}", y);  // Should result in an error because the String
                        // pointer for each character is moved to z
    println!("{}", z);

}
