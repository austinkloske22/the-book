
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let _s: &'static str = "I have a static lifetime.";
    {
        let r;

        //{ -> Error borrowed value does not live long enough
        let x = 5;
        r = &x;
        //}

        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyzabc";

        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
    
    struct_liftime();

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn struct_liftime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
