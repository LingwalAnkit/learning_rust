#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        // self is a reference to the instance of the struct Rectangle. It allows us to access the fields of the struct and perform calculations based on those fields.
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    let sq = Rectangle::square(5); // associated function

    let result = Rectangle::can_hold(
        &rect,
        &Rectangle {
            width: 20,
            height: 30,
        },
    ); // associated function

    println!("{}", result);

    println!(
        "The area of the square {:#?} is {}",
        sq,
        sq.calculate_area()
    );

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    println!(
        "rectangle {:#?} can hold rectangle {:#?} {}",
        rect,
        rect2,
        rect.can_hold(&rect2)
    ); // here we are calling the method can_hold on the instance of the struct Rectangle. The method takes a reference to another instance of the struct Rectangle and checks if the current instance can hold the other instance based on their width and height fields. The result is then printed to the console.

    println!(
        "The area of the rectangle {:#?} is {}",
        rect,
        rect.calculate_area()
    ); // here we are calling the method calculate_area on the instance of the struct Rectangle. The method takes a reference to the instance of the struct and calculates the area based on the width and height fields of the struct. The result is then printed to the console.
}
