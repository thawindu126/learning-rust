use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

fn main() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    route(&home);
    route(&loopback);

    println!();

    let quit_m = Message::Quit;
    let move_m = Message::Move { x: 2, y: 12 };
    let write_m = Message::Write(String::from("hello"));
    let change_color_m = Message::ChangeColor(245, 22, 149);

    quit_m.call();
    move_m.call();
    write_m.call();
    change_color_m.call();

    let num1: i8 = 5;
    let num2: Option<i8> = Some(5);

    let sum = num1 + num2.unwrap_or(10);
    println!("{}", sum);
}

/* #[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn route(ip_addr: &IpAddr) {
    println!("Routing {:#?}", ip_addr);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self)
    }
}
