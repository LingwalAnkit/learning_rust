fn main() {
    let mut name = String::from("Ankit");
    let length = calculate_length(&name);
    println!("The length of string: {} is {}", name, length);

    let length_mut = calculate_length_mut(&mut name);
    println!("The length of string: {} is {}", name, length_mut);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn calculate_length_mut(s: &mut String) -> usize {
    s.push_str(" Singh");
    s.len()
}
