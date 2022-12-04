use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct SecretNumber {
    pub min: u32,
    pub max: u32,
    pub exist: u32,
    pub show_result: bool,
}

impl SecretNumber {
    fn square(max: u32, show_result: bool) -> SecretNumber {
        SecretNumber {
            min: 1,
            max,
            exist: 0,
            show_result,
        }
    }
    pub fn start(&self) {
        println!("猜数开始!");

        let secret_number = rand::thread_rng().gen_range(self.min..self.max);

        if self.show_result {
            println!("要猜的数是: {}", secret_number);
        }

        println!("请输入一个数{}-{}之间的整数，输入0放弃继续猜数", self.min, self.max);
        println!();

        let mut count = 0;
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

            // match guess.cmp(&secret_number) {
            //     Ordering::Less => println!("猜小了!"),
            //     Ordering::Greater => println!("猜大了!"),
            //     Ordering::Equal => {
            //         println!("猜对了!");
            //         break;
            //     }
            // }

            print!("您输入的数字是: {}。 ", guess);
            if guess == self.exist {
                println!("您放弃了");
                break;
            } else if guess == secret_number {
                println!("恭喜您猜对了。您猜了 {} 次", count);
                break;
            } else if guess > secret_number {
                println!("猜大了");
                count = count + 1;
            } else if guess < secret_number {
                println!("猜小了");
                count = count + 1;
            }
        }
    }
}