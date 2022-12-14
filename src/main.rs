use std::{io, process};
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("Please input your guess....");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();
        /*
         *  ARTICLE
         *  http://danielnill.com/rust_tip_compairing_strings
         */

        if guess == "quit" {
            println!("Thanks for playing the game!");
            process::exit(0);
        }
        
        // let guess: u32 = guess.trim().parse().expect("Please enter a valid number");
        let guess: u32 = match guess.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Expected a valid number. Please try again");
                continue;
            }
        };

        /* 
            If parse returns an Err Result variant because it 
            couldn’t create a number from the string, the expect call 
            will crash the game and print the message we give it. 
            If parse can successfully convert the string to a number, 
            it will return the Ok variant of Result, and expect will 
            return the number that we want from the Ok value.
        */
        
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Your guess was lesser"),
            std::cmp::Ordering::Greater => println!("Your guess was greater"),
            std::cmp::Ordering::Equal => {
                println!("Congratulations!!! Your guess {guess} was correct!!");
                break;
            }
        }
        
        /*
            In the match block, the first expression should evaluate.
            It's corresponding possible values are then evaluated
            Compare it with JS Switch
            switch (value we are switching against) {
                case "possible expected value"
            } 
            But since we know the cmp is only to return values of the Ordering ENUM,
            we can directly infer these possible outcomes
            using Ordering::Less or e.t.c   
        */
    }
}

/*
    Statements are instructions that perform some action and do not return a value. 
    Expressions evaluate to a resulting value. Let’s look at some examples.

    Expressions do not include ending semicolons. 
    If you add a semicolon to the end of an expression, you turn it into a statement, 
    and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
*/