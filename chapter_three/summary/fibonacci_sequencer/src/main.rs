use std::io;

fn main() {
    println!("\n Enter an Fibonacci Sequence index");
    let mut integer_input = String::new();

    io::stdin()
        .read_line(&mut integer_input)
        .expect("Failed to read line");
        
    let integer: u32 = match integer_input.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            panic!( "That wasn't valid input! Index must be a positive integer" )
        },
    };

    println!("You entered: {}", integer);
    iterate(integer);
}

fn fib (n: u32) -> u32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fib (n-1)  + fib(n-2);
 }
}

fn iterate(x: u32) {
    // let mut fibonacci_values: (i32, i32) = (0, 1);
    for index in 0..x {
        println! ( "fibonacci ({}) => {}", index, fib(index));
    }
}
