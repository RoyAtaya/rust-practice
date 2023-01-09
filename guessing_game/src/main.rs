use std::io::{self, Write}; // std::io is the guess/output library
// need to include Write for stdout().flush().unwrap() and print! to work
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game. The purpose of the game is for you to guess what number the computer is thinking of.\n");
    let random_number =rand::thread_rng().gen_range(1..=100);

    loop{
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        /*
        Rust allows us to shadow the previous value of guess with a new one. 
        Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess,
        */

        match guess.cmp(&random_number){
            Ordering::Equal => {
                println!("Congrats, you guessed the correct number. Your guess was {guess} and the secret number was {random_number}.");
                break;    
            },
            Ordering::Less => println!("Sorry, you did not guess the correct number. Your guess was too small.\n"),
            Ordering::Greater => println!("Sorry, you did not guess the correct number. Your guess was too large.\n"),
        }
    }
}
