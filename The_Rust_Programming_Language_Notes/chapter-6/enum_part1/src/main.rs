struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind{
    V4,
    V6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}



fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn IpExample(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}

fn route(ip_type: IpAddrKind) { }

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
