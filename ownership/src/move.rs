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
