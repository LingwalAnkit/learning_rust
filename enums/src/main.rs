#[derive(Debug)]
enum IPKind {
    IPv4,
    IPv6,
}

#[derive(Debug)]
struct IPAddress {
    address: String,
    kind: IPKind,
}

impl IPAddress {
    fn route(&self) {
        println!("Routing to {:?} IP address: {}", &self.kind, &self.address);
    }

    fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            kind: IPKind::IPv4,
        }
    }
}

fn main() {
    let ipadd = IPAddress::new("123.3.4.5");

    ipadd.route(); // method call
}
