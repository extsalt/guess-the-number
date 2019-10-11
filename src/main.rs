extern crate rand;

use std::io;

use rand::Rng;
fn main() {
    println!("Guess the number");

    println!("Please enter your guess:");

    let mut guess = String::new(); //create new string variable

    //read user input
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let random = rand::thread_rng().gen_range(1,100);

    println!("Secret Number: {}", random);
}
