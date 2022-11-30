use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数开始!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("要猜的数是: {}", secret_number);

    println!("请输入一个数1-100之间的正数，输入0放弃继续猜数");
    println!();

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}是无效输入请继续输入", guess);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了!"),
            Ordering::Greater => println!("猜大了!"),
            Ordering::Equal => {
                println!("猜对了!");
                break;
            }
        }

        print!("您输入的数字是: {}. ", guess);
        if guess == 0 {
            println!("你放弃了");
            break;
        } else if guess == secret_number {
            println!("猜对了");
            break;
        } else if guess > secret_number {
            println!("猜大了");
        } else if guess < secret_number {
            println!("猜小了");
        }
    }
}
