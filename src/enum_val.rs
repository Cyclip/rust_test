enum IP {
    V4(String),
    V6(String)
}

impl IP {
    // Get the ip value
    fn get_ip(&self) -> &String {
        match self {
            IP::V4(ip) | IP::V6(ip) => ip
        }
    }
}

fn main() {
    let a = IP::V4(String::from("127.0.0.1"));
    let b = IP::V6(String::from("4613:4650:2932:7cc8:ad3d:15b8:0187:d1d4"));

    println!("a: {}", a.get_ip());
    println!("b: {}", b.get_ip());
}
