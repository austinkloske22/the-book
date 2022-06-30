#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Square Factory 
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

/*
fn area(input: &Rectangle) -> u32 {
    input.width * input.height
}
*/

fn main() {

    let rectangle1 = Rectangle {
        width: 30,
        height: 50
    };

    let small_rect = Rectangle {
        width: 10,
        height: 10
    };

    let big_rect = Rectangle {
        width: 50,
        height: 50
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );



    println!("rect1 (:? format) is {:?}", rectangle1.area());
    println!("rect1 (:#? format) is {:#?}", rectangle1.area());
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);


    if rectangle1.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle1.width);
    }

    println!("Rectable1 can hold small rectangle: {}", rectangle1.can_hold(&small_rect));
    println!("Rectable1 can hold big rectangle: {}", rectangle1.can_hold(&big_rect));

    let square = Rectangle::square(3);
    println!("The square has a width of {}", square.width);
}