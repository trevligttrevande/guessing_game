extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // Generate an integer between 1 and 100
    let mut guesses: Vec<(u32,String)> = Vec::new();
    let mut number_of_guesses = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        number_of_guesses += 1; // Increment number_of_guesses each loop
        let guess_tuple: (u32,String) = (number_of_guesses,guess.clone());
        guesses.push(guess_tuple);

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
                for number in 0..2 {
                    for element in guesses.iter() {                                     // Iterators are convinient
                        println!("Guess number {} was {}",element.0, element.1 )
                    }
                }
                break;
            }
        }
    }
}
