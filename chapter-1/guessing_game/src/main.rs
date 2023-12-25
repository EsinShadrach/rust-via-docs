use rand::Rng;
use std::{cmp::Ordering, io};

// Guessing Game
fn main() {
    println!("Make a guess from 1 - 100");

    let mut trials = 5;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Don't tell anyone but this is the secret number: {secret_number}");

    loop {
        println!("Make a guess");

        println!("You have {trials} trials left");

        let mut guess = String::new();

        if trials == 0 {
            break;
        }
        // Since the program breaks here no need for an  `else if`
        if trials == 1 {
            println!("You have one more tial left! Make it count!");
        }

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Guess was re-assigned as an integer now
        let guess = match guess.trim().parse::<u32>() {
            // Okay get's returned here
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid `Number`!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Take another guess"),
            Ordering::Equal => {
                println!("You got it right this time! {guess}");
                break;
            }
            Ordering::Greater => println!("Too high! are you even trying?!"),
        }

        trials -= 1;
    }
}
