// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum IpAddrKind {
//     V4(String), // String値を紐付け
//     V6(String), // String値を紐付け
// }

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

// fn route (ip_type: IpAddrKind){
// }

enum Message {
    Quite,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // let four = IpAddrKind::V4(String::from("127.0.0.1"));
    // let six = IpAddrKind::V6(String::from("::1"));
    
    // route(four);
    // route(six);

    let four = IpAddr::V4(127,0,0,1);
    let six = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    let coin = Coin::Penny;
    println!("{}",value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",six);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
}

