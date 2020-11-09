fn main() {
    println!("Hello, world!");
    let value1 = value_in_cents(Coin::Quarter(UsState::Alabama));

    let coin2 = Coin::Quarter(UsState::Alabama);
    let value2 = coin2.value_in_cents();
    println!(
        "value 1: {:?}, coin 2: {:?}, value 2: {:?}",
        value1, coin2, value2
    );
    let value3 = value2;
    println!("value 3: {:?}", value3);

    let five = Some(5);
    let five_plus_one = plus_one(five);
    let none = plus_one(None);
    println!(
        "five: {:?}, five_plus_one: {:?}, none: {:?}",
        five, five_plus_one, none
    );

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (println!("unknown number")),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state: {:?}", state);
            25
        }
    }
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        //todo  如果没有参数， us state 那里， &self *self   都是可以的
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state: {:?}", state);
                25
            }
        }
    }
}

// 注意这里的
impl Coin {
    fn value_in_cents_todo(&self) -> u8 {
        match self {
            Penny => 1,
            Nickel => 5,
            Dime => 10,
            Quarter => 25,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
