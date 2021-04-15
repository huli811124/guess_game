use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("猜数！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //正式游戏不会打印神秘数字
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("请猜一个数：");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取行失败！");
        //转换  将字符串解析成数字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你才猜测的数字是 {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("数字太小"),
            Ordering::Greater => println!("数字太大"),
            Ordering::Equal => {
                print!("我们赢了");
                break;
            }
        }
    }
}
