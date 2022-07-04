

fn main() {
    vectors();
    enum_vectors();
    strings();
    hash_maps()
}

fn hash_maps() {
    use std::collections::HashMap;

    //Scores
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("scores: {:?}", scores);
    }

    // Teams
    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // scores = scores2 
        let mut _scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(&field_name, field_value);
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
        //println!("field_name is {}", field_name); // Error -> value borrowed here after move

        println!("My favorite color is {:?}", map.get(&field_name));
    }


    //Word Count
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}

fn strings() {
    // data is a string literal
    let data = "initial contents";
    //s1 & s2 are equivelent
    let _s1 = data.to_string();
    let _s2 = String::from("initial contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    
    let hello = String::from("Здравствуйте");
    println!("the length of Здравствуйте is {} bytes", hello.len());
    
    let hello = String::from("Hola");
    println!("the length of Hola is {} bytes", hello.len());

    // println!("नमस्ते as index integers"); runtime error
    // let hello = "नमस्ते";
    //println!("{}", &hello[0..1]);
    //println!("{}", &hello[1..2]);
    //println!("{}", &hello[2..3]);
    //println!("{}", &hello[3..4]);


    println!("नमस्ते as chars:");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    println!("नमस्ते as bytes:");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("s3 is equal to {}", s3);


    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let _game = format!("{}-{}-{}", tic, tac, toe);
}

fn enum_vectors() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn vectors() {
    // new vector
    let mut v1: Vec<i32> = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // utilize vec! macro
    let mut v2 = vec![1, 2, 3, 4, 5];

    {
        // Two ways to get the Third element of a vector
        let third_element: &i32 = &v2[2];
        println!("The third element is {}", third_element);

        match v2.get(2) {
            Some(third_element) => println!("The third element is {}", third_element),
            None => println!("There is no third element."),
        }
    }

    {
        // Two ways to get the 10th element of a vector (element doesn't exist)
        //let tenth_element: &i32 = &v2[10]; -> Error packick
        //println!("The third element is {}", tenth_element);

        match v2.get(10) {
            Some(tenth_element) => println!("The third element is {}", tenth_element),
            None => println!("There is no third element."),
        }
    }

    // modify vector
    {
        for i in &mut v2 {
            *i += 1;
        }
    }

    // print vector
    for i in &v2 {
        println!("The element is equal to {}", i);
    }

    // print vector and index
    for (i, el) in v2.iter().enumerate() {
        println!("The {} index has a value of {}", i, el);
    }
    
    v2.clear();
    assert!(v2.is_empty());
    
}
