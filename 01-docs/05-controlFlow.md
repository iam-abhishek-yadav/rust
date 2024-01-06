# Rust Control Flow

## Introduction
Control flow in Rust involves managing the execution of code based on conditions and loops. Key constructs include `if` expressions and various types of loops.

## if Expressions
An `if` expression allows conditional branching. It consists of a condition followed by code blocks for true and false outcomes. Optionally, an `else` expression provides an alternative block of code.

```rust
fn main() {
    let number = 3;

    // Example 1: Simple if-else expression
    if number < 5 {
        println!("Example 1: Condition was true");
    } else {
        println!("Example 1: Condition was false");
    }

    // Example 2: Incorrect usage of if without a boolean condition
    // Uncommenting the code below will result in a compilation error
    // if number {
    //     println!("Example 2: Number was three");
    // }

    // Example 3: if with a condition checking if number is not equal to zero
    if number != 0 {
        println!("Example 3: Number was something other than zero");
    }
}
```

## Handling Multiple Conditions with `else if`
Multiple conditions can be handled using `else if` expressions, providing a series of conditions to check.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## Using `if` in a `let` Statement
Since `if` is an expression, it can be used in a `let` statement to assign values based on conditions.

```rust
fn main() {
    // Example 1: Assigning the result of an if expression to a variable
    let condition1 = true;
    let number1 = if condition1 { 5 } else { 6 };
    println!("Example 1: The value of number is: {number1}");

    // Example 2: Assigning the result of an if expression with mismatched types
    // Uncommenting the code below will result in a compilation error
    // let condition2 = true;
    // let number2 = if condition2 { 5 } else { "six" };
    // println!("Example 2: The value of number is: {number2}");
}
```

## Repetition with Loops
Loops in Rust include `loop`, `while`, and `for`. They allow repeated execution of code until a condition is met.

### Repeating Code with `loop`
The `loop` keyword instructs Rust to execute a block of code indefinitely until explicitly stopped.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

### Returning Values from Loops
A loop can return a value using the `break` keyword. The value is specified after `break` and is then assigned to a variable.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### Loop Labels to Disambiguate Between Multiple Loops
Loop labels, starting with a single quote, help disambiguate between nested loops. `break` and `continue` can then specify the labeled loop.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### Conditional Loops with `while`
The `while` loop runs code while a condition holds true. The loop exits when the condition becomes false.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

### Looping Through a Collection with `for`
The `for` loop iterates over elements in a collection, providing a concise and safe way to handle repetition.

```rust
fn main() {
    // Example 1: Countdown using a for loop and range
    for number in (1..4).rev() {
        println!("Example 1: {number}!");
    }
    println!("Example 1: LIFTOFF!!!");

    // Example 2: Looping through each element of an array using a for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("Example 2: the value is: {element}");
    }
}
```

## Conclusion
Control flow constructs in Rust, such as `if` expressions and loops, enable flexible and efficient program execution based on conditions and repetitive tasks.
