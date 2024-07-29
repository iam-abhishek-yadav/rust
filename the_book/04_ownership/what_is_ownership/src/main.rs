fn main() {
    // ------ Ownership rules ------
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped
    
    { // s is not valid here, it's not yet declared
        let _s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // y is a copy of x
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("{}, world!", s1);

    let s = String::from("hello");  // s comes into scope
    // takes_ownership(s);              // s's value moves into the function...
    println!("i take ownership of {}", s); // ... and so is no longer valid here


    let x = 5;
    makes_copy(x);
    println!("x = {}", x);

    let s3 = gives_ownership();
    println!("{}", s3);

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("{}", s5);
}

fn _takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}   

