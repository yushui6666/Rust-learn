use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let snum = rand::rng().random_range(1..=100);
    println!("猜一个 1-100 之间的数字:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                continue;
            }
        };

        println!("你输入的是: {}", guess);

        match guess.cmp(&snum) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你猜对了!");
                break;
            }
        }
    }
}
