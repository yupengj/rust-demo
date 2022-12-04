mod secret_number;
mod user;

fn main() {
    // 用户测试
    let user = user::build_user(String::from("aa"), String::from("aa"));
    dbg!(user);

    // 猜数测试
    secret_number::SecretNumber {
        min: 200,
        max: 5003,
        exist: 0,
        show_result: false,
    }.start();
}
