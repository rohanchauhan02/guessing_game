use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secrect number is: {}", secret_number);
    loop {
        println!("Please enter your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","Wrong Input".red());
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
            Ordering::Greater => println!("{}","Too big!".red()),
        }
    }
}
