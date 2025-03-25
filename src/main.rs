use std::io; // import the io library to manage input/output
use std::cmp::Ordering; // import the cmp library to compare values
use rand::Rng; // import the rand library to generate random numbers

fn main() { //fn syntax declares a new function
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1..=100); //generate a random number between 1 and 100

    println!("The secret number is: {secret_number}"); //display the secret number to the screen - to delete in the future after dev

    println!("Please input your guess.");
    // println! is a macro that prints a string to the screen

    let mut guess = String::new(); //create a mutable variable to store the user input

    io::stdin() //call the stdin function from the io module
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),

        }
        
    }
