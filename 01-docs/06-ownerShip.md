# Ownership in Rust

## What Is Ownership?
Ownership is a set of rules that govern how a Rust program manages memory. It ensures memory safety without relying on a garbage collector. Rust uses a system of ownership with strict rules, and the compiler enforces these rules to prevent memory-related bugs.

## Key Concepts

### 1. Ownership Rules
   - **Unique Ownership:** Each value in Rust has a single owner.
   - **Single Ownership:** A value can have only one owner at a time.
   - **Ownership Drop:** When the owner variable goes out of scope, the associated value is automatically dropped.

### 2. Variable Scope
   - A variable is valid from its declaration point until it goes out of scope.
   - Scopes are delimited by curly braces `{}`.

```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

## Variables and Data Interacting with Move
   - When a value is assigned to another variable, ownership is moved.
   - Move semantics prevent double-free errors.
   - String data is moved, and the original owner becomes invalid.
   - Deep copying requires the `clone` method.

```rust
fn main() {
    // Example 1
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Example 2
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Example 3
    let s1 = String::from("hello");
    let s2 = s1;

    // Uncommenting the line below would result in a compilation error
    // println!("{}, world!", s1);
}

```

## Stack-Only Data: Copy
   - Types implementing the `Copy` trait allow for trivial copying.
   - Copy is applicable to types with a known size at compile time.
   - Types like integers, booleans, and tuples (if their elements are Copy) implement Copy.

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

## Ownership and Functions
   - Passing a value to a function moves or copies ownership.
   - Ownership transfer follows similar rules to variable assignment.
   - Functions can return ownership of values.
   - Returning a value can transfer ownership out of the function.

```rust
fn main() {
    // Example 1
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    // Example 2
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.
}

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
## Borrowing and References

To avoid transferring ownership, Rust introduces borrowing and references. References allow a function to access a value without taking ownership. The `&` symbol is used to create references.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Mutable References

Rust enforces strict rules for mutable references. You can't have more than one mutable reference to a value at the same time. This prevents data races at compile time. This rule ensures controlled mutation.

```rust
fn main() {
    // Example 1
    let mut s = String::from("hello");
    change(&mut s);

    // Example 2 - Uncomment the following lines to see the compilation error
    /*
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */

    // Example 3
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // Example 4 - Uncomment the following lines to see the compilation error
    /*
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);
    */

    // Example 5
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Dangling References

Rust guarantees that references will never be dangling references, which can occur in languages with pointers. The compiler prevents functions from returning references to values that go out of scope. Dangling references can lead to undefined behavior.

To avoid dangling references, Rust encourages returning owned values. This ensures that the data will not go out of scope before the reference to the data does.

```rust
fn main() {
    // Example 1 - Uncomment the following lines to see the compilation error
    /*
    let reference_to_nothing = dangle();
    */

    // Example 2
    let no_dangle_result = no_dangle();
}

// Example 1
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

// Example 2
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```