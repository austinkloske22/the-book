pub trait Coordinates {
    fn distance_from_origin(&self) -> String;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T>Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct DuelPoint<T, U> {
    x: T,
    y: U,
}


fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = largest(&number_list);
        println!("The largest number is {}", largest);
    }
    {
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let largest = largest(&number_list);
        println!("The largest number is {}", largest);
    }
    {
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    {
        generic_struct();
    }
}

fn generic_struct() {
    let _integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    
    println!("p.x = {}", float.x());
    float.distance_from_origin();
    // integer.distance_from_origin();

    // let wont_work = Point { x: 5, y: 4.0 };
    let _will_work = DuelPoint { x: 5, y: 4.0 };
    
}
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
/*
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/