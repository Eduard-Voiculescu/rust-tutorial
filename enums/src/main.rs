use std::os::macos::raw::stat;

enum IpAddr {
    V4 (u8, u8, u8, u8),
    V6 (String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor (i32, i32, i32)
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska
    // insert other states, only using 2 for this example
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin : Coin) -> u8 {
     return match coin {
         Coin::Penny => {
             println!("Lucky penny");
             1
         },
         Coin::Nickel => 5,
         Coin::Dime => 10,
         Coin::Quarter(state) => {
             println!("State quarter from {:?}!", state);
             25
         }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a String");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // this won't compile, because we are trying to add i8 to a Option<i8>
    let sum = x + y.unwrap();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
