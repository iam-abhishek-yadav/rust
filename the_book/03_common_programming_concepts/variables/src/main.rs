fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y is: {}", y);
    let y = "six";
    println!("The value of y is: {}", y);

    const COUNT: u32 = 100_000;
    println!("The value of COUNT is: {}", COUNT);
}
