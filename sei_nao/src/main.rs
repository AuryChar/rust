extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 11);

    loop {
        println!("Guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error on input detect");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Very low"),
            Ordering::Greater => println!("Very High"),
            Ordering::Equal => {
                println!("You got it right! The secret number was: {}", secret_number);
                println!("End :3");
                break;
            }
        }
    }
}
