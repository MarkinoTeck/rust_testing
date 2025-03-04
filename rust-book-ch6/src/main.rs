enum IpAddrKing {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_func() {
        println!("yes.")
    }
}

struct IpAddr {
    kind: IpAddrKing,
    adress: String
}

fn main() {
    let four = IpAddrKing::V4;
    let six = IpAddrKing::V6;

    let localhost = IpAddrKing::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKing) {
    //
}