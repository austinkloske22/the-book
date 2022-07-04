use std::{io};

fn main() {
   
    println!("Let's calculate mode and medium from a list of Integers");
    println!("How many integers would you like to include?");

    let mut list_length = String::new();
    io::stdin()
        .read_line(&mut list_length)
        .expect("Failed to read line");
    
    let list_length: u32 = match list_length.trim().parse() {
        Ok(num) => num,
        Err(_) => 5,
    };

    let mut vector: Vec<u32> = Vec::new();

    for i in 0..list_length {

        println!("Enter {} more number(s)", (list_length - i));
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        vector.push(input);
    }

    vector.sort();
    median(&vector);
    mode(&vector);

}

fn median(v: &Vec<u32>) {
    println!("Vector length is {}", v.len());
    println!("Vector contents are {:?}", v);
    let median_position = v.len() / 2;
    println!("Median value is: {}", v[median_position]);

}

fn mode(v: &Vec<u32>) {
    use std::collections::HashMap;
    {
        let mut map = HashMap::new();
        for i in v {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}