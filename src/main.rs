use std::io; // import the io library to manage input/output

fn main() { //fn syntax declares a new function
    println!("Guess the number!");

    println!("Please input your guess.");
    // println! is a macro that prints a string to the screen

    let mut guess = String::new(); //create a mutable variable to store the user input

    io::stdin() //call the stdin function from the io module
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
