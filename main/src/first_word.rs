// Problem:
// write a function that takes a string and returns the first word it finds in that string. If the
// function doesn’t find a space in the string, the whole string must be one word, so the entire
// string should be returned

fn main() {
    let x = String::from("hello there!");
    let y = String::from("hi");
    let z = String::from("t e s t");

    let f_x = first_word(&x);
    let f_y = first_word(&y);
    let f_z = first_word(&z);

    println!("{}: {}", x, f_x);
    println!("{}: {}", y, f_y);
    println!("{}: {}", z, f_z);
}

fn first_word(s: &String) -> &str {
    // &String to not take ownership
    // Return a string slice (&str)

    // let mut return_val = String::new();

    // For each char in s, if it isn't space push it to returnVal
    for (i, character) in s.chars().enumerate() {
        if character == ' ' {
            // is space, so end
            return &s[..i];
        }
    }

    // no spaces
    return &s[..];
}
