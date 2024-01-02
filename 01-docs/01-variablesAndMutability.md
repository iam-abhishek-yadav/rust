# Variables and Mutability in Rust

## Immutable Variables

- In Rust, variables are immutable by default.
- Immutable variables cannot be changed once a value is assigned.
- Immutability is a key feature in Rust that enhances safety and concurrency.
- Attempting to reassign a value to an immutable variable will result in a compile-time error.

```
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // Uncommenting this line would result in a compile-time error
}
```

## Using Mutability

- To make a variable mutable, use the `mut` keyword before the variable name.
- Mutable variables can be reassigned to different values.
  
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This is allowed for mutable variables
    println!("The value of x is: {}", x);
}
```

## Constants

- Constants are similar to immutable variables but have additional restrictions.
- Constants must be annotated with their type and are always immutable.
- They can be declared in any scope, including the global scope.
- Constants are useful for values needed globally or in multiple parts of the code.

```
const PI: f32 = 3.14;
```

## Shadowing

- Shadowing allows declaring a new variable with the same name as a previous variable.
- The new variable "shadows" the previous one, and the compiler uses the new variable's value.
- Shadowing is different from mutability as it allows changing the type of the variable.

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```