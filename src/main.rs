use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input you guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read");

    println!(" You gussed: {guess}");


    let x = 5;
    let y = 10;
    println!("x={x} and y + 2 = {}",y+2)

}