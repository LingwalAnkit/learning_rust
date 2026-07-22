struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }

    fn mixup<X, Y>(self, point: Point<X, Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

impl Point<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let float_list = vec![34.0, 50.0, 25.0, 101.1, 65.0];

    let result = largest_number(&list);
    let float_result = largest_number(&float_list);

    println!("The largest number is {}", result);
    println!("The largest float number is {}", float_result);

    let point_a: Point<f64, f64> = Point::new(4.0, 5.0);
    println!(
        "The distance from the origin is {}",
        point_a.calculate_distance()
    );

    let p1 = Point::new(5, 10);
    let p2 = Point::new(10.5, "Hello");
    let p3 = Point::mixup(p1, p2);
    println!("p3: ({}, {})", p3.x, p3.y);
}

fn largest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // used a trait bound to ensure that T implements PartialOrd so that we can compare the items in the list
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            // error here because T does not know the type what if it is a string or a custom struct that does not implement PartialOrd
            largest = item;
        }
    }
    largest
}
