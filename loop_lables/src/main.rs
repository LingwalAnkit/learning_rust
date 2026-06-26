fn main() {
    let mut count = 0;
    let mut counter = 0;
    let result = 'my_loop: loop {
        count = count + 1;
        println!("Count: {}", count);
        if count == 10 {
            break count * 2;
        }

        loop {
            counter = counter + 2;
            println!("Inner Count: {}", counter);
            if counter == 20 {
                break 'my_loop 0; // breaks out of the outer loop and returns 0
            }
        }
    };
    println!("Result: {}", result);
}
