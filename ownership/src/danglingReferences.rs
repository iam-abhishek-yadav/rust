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
