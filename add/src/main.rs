fn main() {
    let a = add_random().wrapping_add(100);
    // warapping_add will not panic on overflow, it will wrap around instead
    println!("The value of a is: {}", a);

    let b = add_random().checked_add(100);
    // checked_add will return None on overflow
    match b {
        Some(value) => println!("The value of b is {}", value),
        None => println!("Overflow occurred when adding 100 to b"),
    }

    let (c, overflow) = add_random().overflowing_add(100);
    // overflowing_add will return a tuple with the result and a boolean indicating if overflow occurred
    println!("The value of c is: {} , overflow occurred: {}", c, overflow);
}

fn add_random() -> u8 {
    200
}
