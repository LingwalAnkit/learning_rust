fn main() {
    let tup: (i32, f64, u8) = (500, 6.7, 1);
    println!("The value of the tuple is: {:?}", tup);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this

    let (x, y, z) = tup;
    println!("the value of x is : {}", x);
    println!("the value of y is : {}", y);
    println!("the value of z is : {}", z);
}
