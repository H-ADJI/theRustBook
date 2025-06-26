use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Please guess a number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number !");
                continue;
            }
        };
        println!("You guessed {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!(" too small"),
            Ordering::Equal => {
                println!("You Guessed right !");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
