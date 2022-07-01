enum IpAddrKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


// Null concept doesn't need to be brought into scope explicityly
/*
enum Option<T> {
    None,
    Some(T),
}
*/

fn main() {

    concise_control();

    let home = IpAddress {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddress {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route(IpAddrKind::V4); // = route(four);
    route(IpAddrKind::V6); // = route(six);


    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let nickle = Coin::Nickel;
    println!("A Nickle is worth {} cents", value_in_cents(nickle));
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    //Dice game
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
        // other => move_player(other),
    }
    
}

fn route(ip_kind: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}


// Concise Control Flow with if let

fn concise_control() {
    
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Is the same logic as if let but losing exhausting check
    
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }


}