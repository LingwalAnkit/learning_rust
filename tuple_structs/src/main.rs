struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    get_color(red);
}

fn get_color(c: Color) {
    println!("Color: {}, {}, {}", c.0, c.1, c.2);
}
