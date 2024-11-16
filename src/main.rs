use std::io;
use rand::Rng;

fn main() {
    println!("猜数游戏!");
    let mut thread_rng = rand::thread_rng();
    let secret_number = thread_rng.gen_range(1..=101);
    println!("神秘数字是：{}", secret_number);
    println!("猜数： ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    
    println!("你猜的数字是：{}", guess);
}