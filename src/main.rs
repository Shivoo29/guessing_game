use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input you guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read");

    println!(" You gussed: {guess}");

}