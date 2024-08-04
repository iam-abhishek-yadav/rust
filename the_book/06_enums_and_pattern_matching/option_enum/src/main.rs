// enum Option<T> {
//         Some(T),
//         None,
//     }

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);

    let sum = x + y; // cannot add `Option<{integer}>` to `{integer}` the trait `Add<Option<{integer}>>` is not implemented for `{integer}`

    let sum = x + y.unwrap_or(0);
}
