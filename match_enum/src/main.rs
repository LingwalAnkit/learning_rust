#[derive(Debug)]
enum Coins {
    penny,
    nickel,
    dime,
    diamond,
}

fn main() {
    let coin = Coins::dime;
    println!(
        "The value of the {} is {} cents.",
        format!("{:?}", coin),
        value_in_cents(coin)
    );
}

fn value_in_cents(coin: Coins) -> u32 {
    match coin {
        Coins::penny => 1,
        Coins::nickel => 5,
        Coins::dime => 10,
        Coins::diamond => 25,
    }
}
