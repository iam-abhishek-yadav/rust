struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User{
        email: String::from("a@b.com"),
        username: String::from("foo"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    println!("{}", name);
    let email = user1.email;
    println!("{}", email);
    let sign_in_count = user1.sign_in_count;
    println!("{}", sign_in_count);
    let active = user1.active;
    println!("{}", active);

    user1.username = String::from("bar");
    println!("{}", user1.username);

    let user2 = build_user(String::from("b@c.com"), String::from("foobar"));
    println!("{}", user2.username);


    let user3 = User {
        email: String::from("c@d.com"),
        ..user2
    };
    println!("{}", user3.username);
}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
