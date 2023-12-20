// io library in standard library 
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        // `let` makes a variable
        let mut guess = String::new(); // mut : mutable, rust use a imutable variable by default.
    
        io::stdin() // read_line's parameter must be mutable.
            .read_line(&mut guess) // & means reference. and & is immutable by default.
            .expect("Failed to read line"); // read_line return Result. `.expect` has two variant(Ok, Err).
    
        // shadowing
        // `parse()` converts variable' type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        // `cmp` returns Ordering and Ordering has three variant(Less, Greater, Equal)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
