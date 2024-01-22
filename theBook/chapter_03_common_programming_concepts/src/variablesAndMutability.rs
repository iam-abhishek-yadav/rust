fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const COUNT: u32 = 1000;
    println!("{}", COUNT);

    let y: i32 = 5;
    println!("The value of y is {}", y);
    let y: &str = "six";
    println!("The value of y is {}", y);
}
