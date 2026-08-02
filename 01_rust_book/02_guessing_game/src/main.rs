use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Print input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Your current input '{guess}' is incorrect. Please type an unsigned integer!"
                );
                continue;
            }
        };

        println!("You guessed : {guess}");

        if secret_number == guess {
            println!("You guessed correctly!");
            break;
        } else {
            if guess > secret_number {
                println!("Your number is greated than secret number");
            } else {
                println!("Your number is lower than secret number");
            }
        }
    }
}
