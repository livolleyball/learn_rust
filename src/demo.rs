const MAX_LINE: i32 = 100;

fn main() {
    println!("MAX_LINE是：{}  ", MAX_LINE);

    //  整数类型
    let guess: u32 = "42".parse().expect("Not a number!");
    let _guess: i32 = "42".parse().expect("Not a number!");
    println!("_guess 是   {} ", _guess);

    // 浮点类型
    // 不声明时默认是 f64
    let x = 2.0;
    let y: f32 = 2.0;
    println!(" x is  {} , y is {} ", x, y);
    
    

    fn_numeric_perations();
    fn_bool();
    fn_char();
    fn_tuple();

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{}", months[0])
}

fn fn_bool() {
    let t = true;
    let f: bool = false;
    println!("t is {}", t);
}

fn fn_char() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}, {}, and {}", c, z, heart_eyed_cat)
}

fn fn_numeric_perations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("truncated is  {} ", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("remainder is  {} ", remainder);
}


fn fn_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // 点标记法  a period (.) followed by the index of the value we want to access
    let five_hundred = x.0;
    println!("five_hundred is {}", five_hundred)
}