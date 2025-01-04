#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // No data associated.
    Move {x: i32, y: i32},      // Has named fields, like a struct.
    Write(String),              // Include a single String.
    ChangeColor(i32, i32, i32), // Includes 3 i32 values.
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
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

fn value_in_cents(coin: Coin) -> u8 {
    // Match all the patterns for Coin enum.
    // `match` can return any type.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // Extract the value from Some(T), and bind it to i.
    }
}

struct MessageWrapper {
    pub message : Message,
    pub set : bool,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let mes = Message::Write(String::from("hello"));
    mes.call();

    // Option Usage.
    let some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let _x = 5;
    // let sum = x + some_number; // This won't compile as Option<i32> and i32 are different types.

    // Match usage.
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    plus_one(some_number);
    plus_one(None);

    // Catch-all pattern
    let dice_roll = 9;

    // Usage of `other`.
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        other => println!("{other}"),
    }

    // Usage of `_`
    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        _ => println!("Value skipped"),
    }

    // Usage of if let.
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let message_wrapper = MessageWrapper {
        message : Message::Move {x : 1, y : 2},
        set : true,
    };

    match message_wrapper {
        MessageWrapper {
            message : Message::Move { x, y },
            ..
        } => {
            println!("Move to: ({} {})", x, y);
        },
        MessageWrapper { set, .. } => {
            println!("{}", set);
        }
    }

    if let MessageWrapper { message : Message::Move { x, y }, ..} = message_wrapper {
        println!("Move to x: {}, y: {}", x , y)
    }
}
