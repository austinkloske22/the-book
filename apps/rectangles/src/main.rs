#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let rectangle1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rectangle1)
    );



    println!("rect1 (:? format) is {:?}", rectangle1);
    println!("rect1 (:#? format) is {:#?}", rectangle1);
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(input: &Rectangle) -> u32 {
    input.width * input.height
}