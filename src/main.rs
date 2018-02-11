extern crate rand; // external dependency

use std::io;
use std::cmp::Ordering;
use rand::Rng; // Trait

fn main() {
    
    println!("Guess the number!"); // ! <--- indicates macro

    let secret_number = rand::thread_rng().gen_range(1, 101); // values are immutable by default

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: <--- static

        io::stdin().read_line(&mut guess) // & <--- ref
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => { 
                println!("You Win!");
                break;
            }
        }
    }

}