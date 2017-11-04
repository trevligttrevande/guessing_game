extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // Generate an integer between 1 and 100
    let mut guesses  = HashMap::new();
    let mut number_of_guesses = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        number_of_guesses += 1; // Increment number_of_guesses each loop
        guesses.insert(number_of_guesses, guess.clone());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad Input! Very bad!");
                continue;
            }
        };
         
        println!("You guessed: {}", guess);



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! You needed {} guesses!", number_of_guesses);
                for element in guesses {
                    println!("Guess number {} was {} ",element.0,element.1);
                }
                break;
            }
        }
    }
}
