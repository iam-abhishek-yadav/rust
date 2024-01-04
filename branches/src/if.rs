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
