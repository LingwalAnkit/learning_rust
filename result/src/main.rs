use std::fs::File;

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Cannot divide by zero"));
    } else {
        Ok(x / y)
    }
}

fn main() {
    let r: Result<i32, String> = divide(10, 0);
    match r {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => println!("File created successfully: {:?}", fc),
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("something went wrong: {e:?}"),
        },
    }
}
