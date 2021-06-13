#![allow(unused)]

use std::io;
use std::convert::TryFrom;


fn test_function(x: i32, y: i32) -> i32 {
    let mut _x = x;
    while (_x * y) < 5000 {
        _x *= y;
    }
    _x
}


fn build_bar(x: i32, max: i32, character: char, max_chars: i32) -> String {
    // Conver i32 to f32
    let x: f32 = x as f32;
    let max: f32 = max as f32;
    let max_chars: f32 = max_chars as f32;

    // Get multiplier to 1dp
    let multiplier: f32 = x / max;

    // Get chars filled
    let chars_filled: i32 = (max_chars * multiplier) as i32;

    // Repeat char for ^
    let iter = std::iter::repeat(character).take(usize::try_from(chars_filled).unwrap());
    let bar: String = iter.collect();
    bar
}

fn main() {
    for x in 1..151 {
        let result = test_function(x, 2);
        println!("{} ({}): {}", x, result, build_bar(result, 5000, '#', 150));
    }
}
