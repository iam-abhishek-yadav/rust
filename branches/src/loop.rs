fn main() {
    // Example 1: Infinite loop printing "again!"
    loop {
        println!("again!");
    }

    // Example 2: Loop with a counter and breaking with a result
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Example 2: The result is {result}");

    // Example 3: Nested loops with labels and breaking to the outer loop
    let mut count = 0;
    'counting_up: loop {
        println!("Example 3: count = {count}");
        let mut remaining = 10;

        loop {
            println!("Example 3: remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Example 3: End count = {count}");
}
