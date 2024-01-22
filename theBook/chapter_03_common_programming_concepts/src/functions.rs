fn main() {
    print_fn();
    value_fn(11, 22);
    sum(22, 23);
}

fn print_fn() {
    println!("Another function.");
}

fn value_fn(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// The return type is specified as i32.
fn sum(x: i32, y: i32) -> i32 {
    // The result of the addition is implicitly returned without using the 'return' keyword.
    x + y
}
