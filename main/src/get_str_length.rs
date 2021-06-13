#![allow(unused)]

use std::io;
use io::Write;

fn get_length(s: &mut String) -> usize {
    // It is &String because we don't want to take
    // ownership of the string, we just want the
    // pointer.

    // To prove it uses the reference and does not
    // take ownership, we will also push extra
    // text which should be apparent when printing
    // those strings in main(). (must be mutable)
    s.push_str(" world");

    return s.len();
}

fn main() {
    let mut x = String::from("hello");      // owns "hello"
    let mut y = String::from("hi there");   // owns "hi there"

    let len_x = get_length(&mut x);
    let len_y = get_length(&mut y);

    println!("{}: {}", x, len_x);
    println!("{}: {}", y, len_y);

}
