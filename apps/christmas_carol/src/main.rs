fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let gifts = ["a partridge in a pear tree", "Two turtledoves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a-laying",
              "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming", ];
    
    for day in 0..12 {
        println!("\nOn the {} day of Christmas", days[day]);
        println!("My true love sent to me");
        
        for gift in (0..day + 1).rev() {
            if day > 0 && gift == 0 {
                print!("And ");
            }
            println!("{}", gifts[gift]);
        }
    }
}
