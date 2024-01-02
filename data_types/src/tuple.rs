fn main() {
    let _unit_tuple: () = ();
    println!("{:?}", _unit_tuple);

    fn example_function() {
        println!("This function returns unit.");
    }

    let result = example_function();
    println!("The result of the function is: {:?}", result);

    let empty_tuple: () = ();
    println!("The type of empty_tuple is: {:?}", empty_tuple);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
}
