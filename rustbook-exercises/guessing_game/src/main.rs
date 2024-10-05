use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    start_game();
}

fn start_game() {
    println!("Guess a number between 1 and 1000!");

    let random_number = rand::thread_rng().gen_range(1..=1000);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        match guess.trim().parse::<u32>() {
            Ok(number) if number >= 1 && number <= 1000 => {
            
                match number.cmp(&random_number) {
                    Ordering::Less => println!("Too small"),
                    Ordering::Greater => println!("Too large"),
                    Ordering::Equal => {
                        println!("Congratulations! You guessed the correct number: {}", random_number);
                        break; 
                    }
                }
            }
            Ok(_) => {
                println!("Error: Entered number is not in valid range (1-1000).");
            }
            Err(_) => {
                println!("Error: Invalid input! Please enter a valid number.");
            }
        }
    }
}
