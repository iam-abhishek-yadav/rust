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
