fn main() {
    // Examples of integer types
    let my_integer: i32 = 42;
    let my_unsigned_integer: u64 = 100;

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // Floating-point numbers
    let my_float: f64 = 3.14;

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    // Boolean type
    let my_true_bool = true;
    let my_false_bool: bool = false;

    // Character type
    let my_char = 'z';
    let unicode_char: char = '😻';

    // Print values
    println!("Integer types: {} {}", my_integer, my_unsigned_integer);
    println!("Integer literals: {} {} {} {} {}", decimal, hex, octal, binary, byte);
    println!("Floating-point number: {}", my_float);
    println!("Numeric operations: {} {} {} {} {} {}", sum, difference, product, quotient, truncated, remainder);
    println!("Boolean types: {} {}", my_true_bool, my_false_bool);
    println!("Character types: {} {}", my_char, unicode_char);
}
