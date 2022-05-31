use std::io;

fn main() {
    println!("\n Enter an Fibonacci Sequence index");
    let mut integer_input = String::new();

    io::stdin()
        .read_line(&mut integer_input)
        .expect("Failed to read line");
        
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let guess: u32 = match integer_input.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            panic!( "That wasn't valid input! Index must be a positive integer" )
        },
    };

    println!("You entered: {}", integer_input);
}
