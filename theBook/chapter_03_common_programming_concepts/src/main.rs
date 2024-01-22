fn main() {
    // Example using a 'loop'
    let mut counter_loop = 0;
    let result_loop = loop {
        counter_loop += 1;
        if counter_loop == 10 {
            break counter_loop;
        }
    };
    println!("Result using 'loop': {}", result_loop);

    // Example using a 'while' loop
    let mut counter_while = 0;
    while counter_while < 10 {
        counter_while += 1;
    }
    println!("Result using 'while': {}", counter_while);

    // Example using a 'for' loop with an iterator
    let mut counter_for_iterator = 0;
    for i in 0..10 {
        // 'i' takes on each value in the range 0..10 in each iteration
        counter_for_iterator += 1;
    }
    println!("Result using 'for' loop with iterator: {}", counter_for_iterator);
}
