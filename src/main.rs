extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses = 0;

    loop {
        println!("Please input your guess.");

        let guess: u32 = match get_guess(){
            Ok(num)=> num,
            Err(e) => {
                println!("ERROR: {}",e);
                continue;
            }
        };
         
        println!("You guessed: {}", guess);

        number_of_guesses += 1; // Increment number_of_guesses each loop

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You needed {} guesses!", number_of_guesses);
                break;
            }
        }
    }
}

// This function handles the input for the guessing game.
// If the input is not an integer it returns an error instead.
fn get_guess()->Result<u32, String> 
{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num)=>Ok(num),
        Err(e) =>{
            let test = format!("{}{}", "In parsing u32, ", e);
            Err(test)
        }
            
        
    }
}