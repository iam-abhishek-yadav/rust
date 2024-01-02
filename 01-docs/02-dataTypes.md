# Rust Data Types Notes

## Introduction
In Rust, every value is associated with a specific data type, indicating the kind of data it represents. Rust categorizes data types into two subsets: **scalar** and **compound**. It's important to note that Rust is a statically typed language, requiring knowledge of variable types during compile time. The compiler often infers types based on values and their usage.

## Scalar Types
Scalar types represent single values and encompass integers, floating-point numbers, Booleans, and characters.

### Integer Types
Integers are whole numbers without fractional components. Rust has four primary integer types: `i8`, `u8`, `i32`, and `u32`, each with explicit sizes. Signed variants (starting with 'i') can represent both positive and negative values, while unsigned variants (starting with 'u') represent only non-negative values.

```
let my_integer: i32 = 42;
let my_unsigned_integer: u64 = 100;

// Integer literals

let decimal = 98_222;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A';
```

### Floating-Point Types
Rust provides `f32` and `f64` as primitive types for floating-point numbers, representing numbers with decimal points. The default type is `f64` due to its comparable speed and increased precision on modern CPUs. All floating-point types in Rust are signed.

```
let my_float: f64 = 3.14;
```

### Numeric Operations
Rust supports standard mathematical operations for all number types, including addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero.

```
let sum = 5 + 10; // Output: 15
let difference = 95.5 - 4.3; // Output: 91.2
let product = 4 * 30; // Output: 120
let quotient = 56.7 / 32.2; // Output: 1.7608695652173911
let truncated = -5 / 3; // Output: -1
let remainder = 43 % 5; // Output: 3
```

### Boolean Type
The Boolean type in Rust, denoted by 'bool', has two possible values: `true` and `false`. Booleans are one byte in size and are commonly used in conditional expressions.

```
let my_true_bool = true;
let my_false_bool: bool = false;
```

### Character Type
Rust's `char` type is the most primitive alphabetic type. It represents Unicode Scalar Values, allowing the inclusion of various characters, symbols, and emojis. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF.

```
let my_char = 'z';
let unicode_char: char = '😻';
```

## Compound Types
Compound types in Rust facilitate grouping multiple values into a single type. The two primitive compound types are **tuples** and **arrays**.

### Tuple Type
Tuples provide a general way to combine values of different types into a single compound type. They have a fixed length, ensuring stability in structure. Tuples are created by listing values separated by commas inside parentheses.

```
fn main() {
    let _unit_tuple: () = ();
    println!("{:?}", _unit_tuple); // Output: ()

    fn example_function() {
        println!("This function returns unit."); // Output: This function returns unit. 
    }

    let result = example_function();
    println!("{:?}", result); // Output: ()

    let empty_tuple: () = ();
    println!("{:?}", empty_tuple); // Output: () 

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}", x); // Output: 500
    println!("{}", y); // Output: 6.4
    println!("{}", z); // Output: 1

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}", five_hundred); // Output: 500
    println!("{}", six_point_four); // Output: 6.4 
    println!("{}", one); // Output: 1
}
```

### Array Type
Arrays, another form of compound type, require all elements to have the same type. They boast a fixed length, making them suitable for situations where the size remains constant. Elements are specified as a comma-separated list inside square brackets.

#### Accessing Array Elements
Array elements are accessed through indexing, starting from 0. Rust's runtime checks prevent accessing elements beyond the array's bounds.

#### Handling Invalid Access
Rust prioritizes memory safety by panicking at runtime when attempting to access elements outside the array's bounds. This behavior prevents invalid memory access.

```
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```