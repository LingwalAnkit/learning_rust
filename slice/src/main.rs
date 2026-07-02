fn main() {
    let mut input = String::from("Hello World");

    let hello = &input[0..5];
    let world = &input[6..11]; // upper bound is not included

    // slices are contiguous sequences of elements in a collection, and they allow you to reference a portion of the collection without taking ownership of it. In this case, we are creating slices of the input string to reference the "Hello" and "World" parts of the string.

    println!("Hello = {}, World = {}", hello, world);

    let result = first_letter(&input);
    input.clear();
    // here i cleared the input string but the result still carries the value of the first space index because the result is a copy of the value and not a reference to the original string. Therefore, even though the original string has been cleared, the result still holds the value of the first space index.
    println!("First space at index: {} of String: {}", result, input);
}

fn first_letter(s: &String) -> usize {
    let input = s.as_bytes();

    for (i, &item) in input.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return input.len();
}
