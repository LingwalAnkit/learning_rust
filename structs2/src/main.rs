#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = (30, 40);

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    let area1 = calculate_area_tuple(rect1);
    println!("The area of the rectangle is: {}", area1);

    let area2 = calculate_area_struct(&rect2);
    println!("The area of the rectangle : {:?} is: {}", rect2, area2);
}

fn calculate_area_tuple(dimensions: (u32, u32)) -> u32 {
    let (width, height) = dimensions;
    width * height
}

fn calculate_area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
