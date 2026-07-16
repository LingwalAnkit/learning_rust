#[derive(Debug)]
enum Spreadsheetcell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vector = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];

    let third_value = &vector[2];
    println!(
        "The third value of vector: {:?} is: {}",
        vector, third_value
    );

    vector.push(11);

    let third_value = vector.get(2);
    match third_value {
        Some(value) => println!("The third value of vector: {:?} is: {}", vector, value),
        None => println!("There is no third value in the vector: {:?}", vector),
    }

    for i in &mut vector {
        println!("{}", i);
        *i = *i * 2;
    }
    println!("The vector after doubling each value: {:?}", vector);

    let spread: Vec<Spreadsheetcell> = vec![
        Spreadsheetcell::Int(20),
        Spreadsheetcell::Float(20.5),
        Spreadsheetcell::Text(String::from("Hello")),
    ];

    println!("The vector of spreadsheet cells: {:?}", spread);

    let element = spread.get(0);
    match element {
        Some(_) => println!(
            "The first element of the vector of spreadsheet cells is: {:?}",
            spread[0]
        ),
        _ => println!("The first element of the vector of spreadsheet cells is not an Int"),
    }
}
