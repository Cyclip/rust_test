#![allow(unused)]

use std::io;
use std::convert::TryFrom;

fn build_layer(n: i32, size: i32) -> String {
    // Build right side first
    let right_filled: String = std::iter::repeat('■')
        .take(usize::try_from(n).unwrap())
        .collect();

    let right_empty: String = std::iter::repeat(' ')
        .take(usize::try_from(size - n).unwrap())
        .collect();

    let right: String = format!("{}{}", right_filled, right_empty);

    // Break right into chars
    let mut left = right.chars();
    left.next();

    // Reverse and change type
    let left = left.rev();

    // Collect into string
    let left: String = left.collect();

    // Combine with right

    format!("{}{}", left, right)
}

fn main() {
    // Make a pyramid
    let size = 100;

    for n in 1..size {
        println!("{}", build_layer(n, size));
    }
}
