use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    let mut input = String::new();

    println!("Enter the index of the element that you want to access (0-4):");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let index: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("Please enter a valid number: {}", error);
            return;
        }
    };

    let element = arr[index as usize];
    // for indexing array, we need to convert the index to usize type

    println!("The value at index {} is: {}", index, element);
}
