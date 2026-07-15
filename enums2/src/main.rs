#[derive(Debug)]

enum IPKind {
    IPv4(String),
    IPv6(String),
}

fn main() {
    let home = IPKind::IPv4(String::from("192.168.1.1"));
    let work = IPKind::IPv6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    println!("{:?}", work);

    route(&home);
}

fn route(ip: &IPKind) {
    println!("Routing to {:?}", ip);
}
