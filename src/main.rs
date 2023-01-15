use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guesses_number: u32 = 0;
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Geuss the number between 1 and 100!");

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(result) => result,
            Err(_) => {
                println!("Could not read input.");
                continue;
            }
        };

        println!("You guessed {guess}.");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not convert input to number, please input a number.");
                continue;
            }
        };
        guesses_number += 1;

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won");
                println!("It took you {guesses_number} guesses");
                break;
            }
        };
    }
}
