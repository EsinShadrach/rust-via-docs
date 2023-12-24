use std::io;

// Guessing Game
fn main() {
    println!("Guessing Game");
    println!("Enter a number from 1-100");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faled to read line");

    println!("You guessed {}", &guess);
}
