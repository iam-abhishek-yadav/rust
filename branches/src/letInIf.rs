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
