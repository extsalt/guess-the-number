use std::io;

fn main() {
    println!("Guess the number");

    println!("Please enter your guess:");

    let mut guess = String::new(); //create new string variable

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
