use std::io;
// use std::io::stdin; 注意包的引用的写法
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("the secret number is {}.", secret_number);
    println!("Plz input your guess:");

    loop {
        //  关联函数（associated function） 静态方法（static method）
        let mut guess = String::new();
        // &引用
        io::stdin()
            .read_line(&mut guess)
            .expect("filed to read line");
        // stdin().read_line( &mut guess).expect("filed to read line");

        // let guess: u32 = guess.trim().parse().expect("Plz enter a number");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            // _ 为通配符
            Err(_)=> {
                println!("invalid input, Plz enter a number to continue:");
                continue;
            }
        };
        println!("You guessed:  {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            // 注意这里是逗号
            Ordering::Equal => {
                println!("whoooooo");
                break;
            }
        }
    }
}
