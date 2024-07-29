fn main() {
    // ----- Rules for References -----
    // 1. At any given time, you can have either one mutable reference
    //    or any number of immutable references.
    // 2. References must always be valid.
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 is a reference to s1
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    let r1 = & s3;
    let r2 = & s3;
    println!("{}, {}", r1, r2);
    let r3 = &mut s3;
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of the String
    length
}

fn change(some_string: &mut String) { // some_string is a reference to a String
    some_string.push_str(", world"); // push_str() appends a literal to a String
}