fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    statements_and_expessions();
    
    let mut x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn statements_and_expessions() {
    // let x = (let y = 6); // -> error: expected expression, found statement (`let`)
    // let x = y = 6; // -> error: not found in this scope

    // Looks like we're passing a set of instructions to variable y
    // Could we read the current environment to execute different logic?
    // I think that this is related to how XCM works... revisit this
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //; But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error.
}
