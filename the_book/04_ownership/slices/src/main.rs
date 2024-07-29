fn main() {
    let s1 = String::from("hello world");
    let hello = &s1[..5];
    let world = &s1[6..];
    let hello_world = &s1[..];
    let s2 = "hello world";
    let word = first_word(s2);
    println!("{} {} {} {}", hello, world, hello_world, word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
