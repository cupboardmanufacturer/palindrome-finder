/*
    # palindrome finder
    # by cupboardmanufacturer
    # contributers:
    #       - AeternusDio: helped to fix a bit of the code
*/

use std::io;
use colored::*;

fn main() {

    let mut guess: String = String::new();

    println!("{}", "palindrome finder!".blue());
    println!("enter a string:");
    io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
                
    let guess = guess.trim().to_lowercase();

    let guess_r: String = guess.chars().rev().collect();

    if guess == guess_r {
        let message = format!("{}: is a palindrome!", guess);
        println!("{}", message.green());
    } else {
        let message = format!("{}: is not a palindrome!", guess);
        println!("{}", message.red());
    }
}
