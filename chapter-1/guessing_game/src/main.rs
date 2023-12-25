use rand::Rng;
use std::{cmp::Ordering, io};

// Guessing Game
fn main() {
    println!("Make a guess from 1 - 100");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Don't tell anyone but this is the secret number: {secret_number}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Guest was re assigned as an integer now
    let guess = guess.trim().parse::<u32>().expect("Failed to convert");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low! Take another guess"),
        Ordering::Equal => println!("You got it right this time! {guess}"),
        Ordering::Greater => println!("Too high! are you even trying?!"),
    }
}
