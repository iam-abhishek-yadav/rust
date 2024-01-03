## Functions in Rust

Functions are integral to Rust programming, serving as building blocks for programs. The `main` function acts as the entry point for many Rust programs. The `fn` keyword is used to declare functions, and Rust adheres to the snake case convention for function and variable names.

### Basic Function Structure

Here, we define a basic function `another_function` that is called from the `main` function. Rust allows flexibility in the order of function definitions.

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Function Parameters

Functions can have parameters, defining inputs to the function. Parameters must declare their types.

This demonstrates a function `another_function` with an `i32` parameter. The function is called with an argument of `5`.

```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

### Statements and Expressions

Rust functions consist of statements and expressions. Statements are instructions without return values, while expressions evaluate to a resultant value.

In this example, the block is an expression evaluating to `4`, assigned to the variable `y`.

```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

### Functions with Return Values

Functions can return values using the `->` syntax. The return value is the result of the final expression.

Here, `five` returns `5`, and `plus_one` takes an `i32` parameter, returning the result of `x + 1`.

```
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

### Error in Statements

It's crucial to avoid semicolons at the end of expressions, as this turns them into statements, preventing them from returning values. Removing the semicolon resolves the mismatched types error.

```
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

These fundamental concepts set the foundation for writing effective Rust code. Understanding functions, parameters, statements, and expressions is crucial for building robust and efficient Rust programs.