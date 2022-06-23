fn main() {

    // Heap data
    {          
        let mut s1 = String::from("hello");
        // do stuff with s
        s1.push_str(", world!"); // push_str() appends a literal to a String
        
        let s2 = s1; // s1 becomes invalid here...

        let s3 = s2.clone(); // s2 remains valid
        let length = calculate_length(&s3);

        // println!("Value of string is {}", s1) Error - > ^^ value borrowed here after move
        println!("Value of string is {}", s2);

        println!("Value of s3 is {}, Length of s3 is {}", s3, length);
    };
    // println!("Value of string is {}", s1) Error -> ^^ not found in this scope


    // Stack data
    {
        let x = 5;
        let y = x;
    
        println!("x = {}, y = {}", x, y);
    }

    //Slice type
    let words = String::from("hello world");
    let word_index = first_word_index(&words);
    println!("The first word index is 0..{}", word_index);

    //String slices
    let s = String::from("hello world");
    let mut hello = &s[0..5];
    // drop 0 also equals hello
    hello = &s[..5];

    let mut world = &s[6..11];
    // or drop the final index to include last byte 
    world = &s[6..];

    let emojis = String::from("😀😇🥰");
    // let emoji = emojis[..2]; // ERROR -> ^^^^^ doesn't have a size known at compile-time

    let mut word = first_word(&words);
    println!("The first word is {}", word);

    let string_literal = "Hello Old World";
    let mut word = first_word(string_literal);
    println!("The first word is {}", word);

    // Array scope
    {
        let array = [1, 2, 3, 4, 5];
        let array_slice = &array[1..3];

        assert_eq!(array_slice, &[2, 3]);
        // assert_eq!(array_slice, &[2, 3, 4]); Error thread 'main' panicked at 'assertion failed: `(left == right)`

        for index in 0..array_slice.len() {
            println!("Array Slice index {} = {}", index, array_slice[index]);
        }
    }
}


//fn first_word(s: &String) -> &str { : Error when passing string literals like "Hello World"
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// return index of space
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// calculate string length using reference
fn calculate_length(s: &String) -> usize {
    s.len()
}
