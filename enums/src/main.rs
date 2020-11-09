fn main() {
    let ip4 = IPAddrCategory::V4;
    let ip6 = IPAddrCategory::V6;
    println!("ip4: {:?}, ip6: {:?}", ip4, ip6);

    let localhost = IPAddr::V4(127, 0, 0, 1);
    let loopback = IPAddr::V6(String::from("::1"));

    println!("localhost: {:?}, loopback: {:?}", localhost, loopback);

    let msg = Message::Hello(String::from("aaron"));
    let call_result = msg.call();
    println!("call result: {:?}", call_result);

    let option_str = Some("aaron");
    let option_int = Some(1);
    println!("option_str: {:?},option_int: {:?}", option_str, option_int)
}

#[derive(Debug)]
enum IPAddrCategory {
    V4,
    V6,
}
#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, //匿名结构体
    Hello(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) -> &Message {
        self
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
