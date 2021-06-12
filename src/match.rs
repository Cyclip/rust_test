// Test program to implement into enums and
// use the match expression

enum Coin {
    Penny,  // 1
    Nickel, // 5
    Dime,   // 10
    Quarter // 25
}

impl Coin {
    // Get coin value in cents in u8
    fn value(&self) -> u8 {
        /*
            Coin::Penny     =>         1
                 ^          ^          ^
              Pattern    Operator  Expression

        Pattern: Compare self to this pattern (Coin::Penny)
        Operator: If it matches, then..
        Expression: Return expression
        */
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn main() {
    let a = Coin::Penny;
    let b = Coin::Dime;

    println!("{}, {}", a.value(), b.value());
}
