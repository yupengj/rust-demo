// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

mod secret_number;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // let email = String::from("someone@example.com");
    // let username = String::from("someusername123");
    // let user1 = build_user(email, username);
    // println!("{:?}", user1);
    // dbg!(&user1);
    //
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    // println!("{:?}", user2);
    // dbg!(&user2);

    let sn = secret_number::SecretNumber {
        min: 200,
        max: 5003,
        exist: 0,
        show_result: false,
    };

    sn.start()
}
