fn main() {
    condition(3);
    condition(7);
    // boolCheck(7).expect("Invalid input"); // QUESTION - how to repllicate try / catch in rust
    // not_zero("Hi");
    not_zero(1);
    not_zero(0);
    multiple_conditions(6);
    let_in_condition(true);
    let_in_condition(false);
    // incompatible_types(true);
}

fn condition(x: i32) {
    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn boolCheck(val: bool) {
    if val {
        println!("number was three");
    }
}

fn not_zero(x: i32) {
    if x != 0 {
        println!("number was something other than zero");
    }
}

// Chapter 6 describes a powerful Rust branching construct called match for these cases.
fn multiple_conditions(x: i32) {
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_in_condition(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

/*
fn incompatible_types(condition: bool) {
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {}", number);
}
*/