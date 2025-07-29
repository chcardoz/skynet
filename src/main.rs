use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut num_guesses = 0;

    loop {
        println!("Guess the number!");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num: u32 = num.trim().parse::<u32>().expect("Please type a number!");
        num_guesses += 1;
        if num < secret_number {
            println!("Too small!");
        } else if num > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            println!("You guessed {num_guesses} times");
            break;
        }
    }
}
