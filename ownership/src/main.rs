fn main() {
    let s = "Hello";
    println!("{}", s);

    let name = String::from("Ankit");
    takes_ownership(name);
    // print!("{}", name); // This line will cause a compile-time error because the ownership of the variable 'name' has been moved to the function 'takes_ownership', and it is no longer accessible in the main function.

    let name2 = gives_ownership();
    println!("{}", name2);

    let name3 = String::from("Ankit");
    let name4 = takes_and_gives_back(name3);
    println!("{}", name4);

    let name5 = String::from("My name is Ankit");
    let (name6, length) = calculate_length(name5);
    println!("The length of '{}' is {}", name6, length);
}

//  variable scope the variable s is valid only within the main function. Once the function ends, the variable s goes out of scope and is no longer accessible. This is an example of ownership in Rust, where variables have a specific scope and lifetime.

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn gives_ownership() -> String {
    let s = String::from("Ownership is given here");
    s
}

// return type is a String

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
