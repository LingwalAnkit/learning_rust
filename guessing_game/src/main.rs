use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading the message");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error parsing the number: {}", err);
                continue;
            }
        };

        // Shadowing the guess variable to make it immutable
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
        }
    }
    // Match is exaustive, so we don't need to handle the other cases
}
