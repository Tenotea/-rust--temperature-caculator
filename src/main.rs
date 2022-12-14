/*
 *  Requirements 
 *  1. What conversion approach are you using?
 *      - Present user with options for different conversion strategies 
 *         1. Fahrenheit to Celsius
 *         2. ...
 * 
 *  2. Compute conversion and display answer in the format
 *      x deg cel is y deg far
 *      - To add spice to this step, we try to display their
 *          symbols in the terminal
 * 
 *  3. Ask if they want to go again (y/n) and quit or restart 
 *     based on response   
 */

use std::io;
fn celsius_to_fahrenheit (degree: i32) -> i32 {
    return (degree * (9/5)) + 32;
}

fn fahrenheit_to_celsius (degree: i32) -> i32 {
    return (degree - 32) * (5/9);
}

fn main() {
    let conversion_strategies: [&str; 3] = [
        "Celsius to Fahrenheit",
        "Fahrenheit to Celsius",
        "Celsius to Kelvin"
    ];

    println!("\nTEMPERATURE CONVERTER #24549 \n--------------------------------------------------------------------\n");
    println!("Please select a conversion strategy");
    for (index, strategy) in conversion_strategies.iter().enumerate() {
        println!("{}. {}", index + 1, strategy.to_string());
    }

    let mut conversion_strategy = String::new();
    io::stdin()
        .read_line(&mut conversion_strategy)
        .expect("The program failed to accept your input. Please restart your terminal and try running the program again");
    
    let mut conversion_strategy: usize = match conversion_strategy.trim().parse() {
        Ok(strategy) => strategy,
        Err(_) => {
            return println!("You have supplied an invalid option");
        }
    };
    conversion_strategy = conversion_strategy - 1;

    if conversion_strategy >= conversion_strategies
                                            .len().try_into()
                                            .expect("Conversion strategies out of range") {
        println!("You have supplied an invalid conversion strategy option");
        return;
    }

    println!("\nEnter your value\n--------------------------------------------------------------------");
    let mut input_degree = String::new();
    io::stdin()
        .read_line(&mut input_degree)
        .expect("The program failed to accept your input. Please restart your terminal and try running the program again");
    
    let input_degree: i32 = match input_degree.trim().parse() {
        Ok(degree) => degree,
        Err(_) => {
            return println!("You have supplied an invalid option");
        }
    };

    let conversion_result = match conversion_strategy {
        0 => celsius_to_fahrenheit(input_degree),
        1 => fahrenheit_to_celsius(input_degree),
        _ => {
            return println!("You have supplied an invalid conversion strategy option")
        }
    };

    println!("Conversion of {} degrees {} is {}", input_degree, conversion_strategies[conversion_strategy], conversion_result)
}
