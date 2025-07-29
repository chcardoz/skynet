use std::cmp::Ordering;
use std::io;

/**
 * Returns a string based on the comparison result of the two numbers.
 * @param num - The number to compare.
 * @param secret_number - The secret number to compare to.
 * @return A string based on the comparison result.
 */
fn get_result_string(num: u32, secret_number: u32) -> String {
    match num.cmp(&secret_number) {
        Ordering::Less => "Too small!".to_string(),
        Ordering::Greater => "Too big!".to_string(),
        Ordering::Equal => "You win!".to_string(),
    }
}

/**
 * Main function.
 * Generates a random number and prompts the user to guess it.
 * Prints the result of the comparison and the number of guesses.
 */
fn main() {
    let secret_number = rand::random::<u32>() % 100 + 1;
    let mut num_guesses = 0;

    loop {
        println!("Guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_result = guess.trim().parse::<u32>();

        let num = match guess_result {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        num_guesses += 1;
        println!("{}", get_result_string(num, secret_number));
        if num == secret_number {
            println!("You guessed {num_guesses} times");
            break;
        }
    }
}
