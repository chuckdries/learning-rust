use std::io;
use io::Write;

fn main() {
    println!("Guess the number!");

    print!("Please input your guess> ");

    io::stdout().flush()
        .expect("Flush failed?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line for some reason");

    println!("You guessed {}", guess);
}
