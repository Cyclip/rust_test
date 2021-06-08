use std::io;
use io::Write;

struct User {
    username: String,
    password: String, // plaintext because hashing is not the point of this test
    shopping_list: Vec<String>,
}

fn main() {
    loop {
        // Login/signup menu
        clear_screen();
        println!(
            "[ MENU ]\n\
            [1] - Register\n\
            [2] - Login\n\
            [3] - Quit"
        );

        match input("Option> ") {
            String::from("1") => {
                menu_register();
            },
            String::from("2") => {
                menu_login();
            },
            String::from("3") => {
                std::process::exit(0);
            }
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

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
    io::stdout().flush().expect("Failed to flush stdout");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line from stdin");

    return trim_newline(&mut inp);
}
