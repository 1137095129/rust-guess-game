use Ord;
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let security = rand::thread_rng().gen_range(1..101);
    println!("Create security number {}", security);
    println!("Guess the number!");
    loop {
        println!("Please input your number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed : {}", guess);
        match guess.cmp(&security) {
            Ordering::Less => println!("less!"),
            Ordering::Equal => {
                println!("congratulations!");
                break;
            }
            Ordering::Greater => println!("greater!")
        }
    }
}
