// Guessing game "Created by AhmadAEngineer"
use rand::{Rng, thread_rng};
use std::io::{stdin, stdout, Write};
use std::cmp::Ordering;

// Defining a method that will generate a random number.
fn random_number() -> u32 {
    let secret_to_guess: u32 = thread_rng().gen_range(1..=10);
    return secret_to_guess;
}


fn main() {
    let mut secret_to_guess: u32;
    println!("*** Guessing Game! ***");
    let mut play_again_choice = true;

    // This is the main loop of the game.
    while play_again_choice {
       secret_to_guess = random_number();
        loop {
            println!("Enter the number to guess (1-10)");

            let mut user_guessed: String = String::new();

            // Taking user input as a string
            match stdin().read_line(&mut user_guessed) {
                Ok(_) => {
                    println!("You have guessed {}", user_guessed)
                }
                Err(err) => {
                    println!("Failed to read your input! {} ", err);
                }
            }

            // Converting user guessed 'string' to a number.
            let user_guessed: u32 = match user_guessed.trim().parse() {
                Err(_) => {
                    println!("Something went wrong!");
                    continue
                }

                Ok(number) => number
            };

            // Comparing both number to check whether to numbers are equals or not.
            match user_guessed.cmp(&secret_to_guess) {
                Ordering::Greater => {
                    println!("Too High!");
                }
                Ordering::Less => {
                    println!("Too Less!");
                }
                Ordering::Equal => {
                    println!("You Win and guessed the right one {}", user_guessed);
                    break;
                }
            }
        }

        // Loop to check whether user want to play again or not.
        loop {
            let mut user_choice: String = String::new();
            println!("Enter '1' to play again and '0' to exit!");

            stdout().flush().expect("Something went wrong!");
            match stdin().read_line(&mut user_choice) { 
                Ok(_) => {
                    match user_choice.trim().parse() {
                        Ok(1) => {
                            println!("Let's play again");
                            break;
                        }
                        Ok(0) => {
                            play_again_choice = false;
                            break;
                        }
                        _ => {
                            println!("Invalid choice please choose '1' or '0'");
                            continue;
                        }
                    }


                    }
                Err(_) => {
                    eprint!("Error while reading!");
                    continue;
                }
            }
        }
    }
}