use std::io;

fn main() {

    println!("\n Enter a tempurature (Format: 32F or 18C):");
    let mut temp_input = String::new();
    
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read line");

    let trimmed = temp_input.trim();

    // Split variables to temp & scale at length -1
    let (temp, scale) = trimmed.split_at(trimmed.len() - 1);

    let temp: f32 = match temp.parse() {
        Ok(num) => num,
        Err(_) => { 
            panic!( "That wasn't valid input! Temperatures must be in format 32F or 18C!" )
        },
    };

    // println!("You entered: {} {}", temp, scale);

    let converted_temp = match scale {
        "C" => celsius_to_fahrenheit(temp),
        "F" => fahrenheit_to_celsius(temp),
        &_ => { 
            panic!( "That wasn't valid input! Temperatures must be in format 32F or 18C!" )
        },
    };

    match scale {
        "C" => println!("{} {}, = {} {}", temp, scale, converted_temp, "F"),
        "F" => println!("{} {}, = {} {}", temp, scale, converted_temp, "C"),
        &_ => { 
            panic!( "That wasn't valid input! Temperatures must be in format 32F or 18C!" )
        },
    };
}

// Fahrenheit to Celsius, subtract 32 and multiply by . 5556 (or 5/9).
fn fahrenheit_to_celsius(x: f32) -> f32 {
    (x-32.0) * ( 5.0 / 9.0 )
}

fn celsius_to_fahrenheit(x: f32) -> f32 {
    (x * 9.0 / 5.0 ) + 32.0
}