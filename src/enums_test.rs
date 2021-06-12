// Enum of type IpAddrKind
// Either v4 or v6 both of type String
enum IpAddrKind {
    V4(String),
    V6(String),
}

// Implement into enum IpAddrKind
impl IpAddrKind {
    // Print values
    fn print(&self) {
        println!("{}", self);
    }
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    home.print();
}
