# Comments in Rust

In Rust, comments are essential for improving code readability. The idiomatic comment style in Rust starts with two slashes, and the comment extends until the end of the line. For multi-line comments, each line must begin with `//`. Comments can also be placed at the end of lines containing code.

## Single-Line Comment
- A simple single-line comment.
```
// hello, world
```

## Multi-Line Comment
```
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

## End-of-Line Comment
```
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```

## Separate Line Comment
```
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```