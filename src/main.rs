use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!");
    let mut thread_rng = rand::thread_rng();
    let secret_number = thread_rng.gen_range(1..=101);
    println!("神秘数字是：{}", secret_number);
    loop {
        println!("请输入一个数字 ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 转化数据类型
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入非法，请输入一个数字");
                continue;
            }
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
