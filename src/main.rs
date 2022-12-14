use std::io;

fn main() {
    println!("\nFIND THE FIBONACCI NUMBER \n----------------------------------------------------\n");
    println!("What position are you looking to find?");

    let mut fibonacci_position = String::new();
    io::stdin()
        .read_line(&mut fibonacci_position)
        .expect("The program failed to accept your input. Consider restarting your terminal and trying again.");
    
    let fibonacci_position: u32 = fibonacci_position
        .trim().parse()
        .expect("Expected a valid positive number");
    
    let mut l_one:usize = 0;
    let mut l_two:usize = 0;
    
    for pos in 0..fibonacci_position {
        if pos > 0 {
            let local_l_one = l_one;
            l_one = l_two;
            l_two = local_l_one + l_two;
        } else {    
            l_two = 1;
        }
    };

    println!("The Fibonacci number at position {} is {}", fibonacci_position, l_two);
}
