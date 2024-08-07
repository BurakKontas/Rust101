#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {

    #[derive(Debug)] // This attribute allows us to print the enum
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);

    fn route (ip_type: IpAddrKind) {
        println!("ip_type: {:?}", ip_type);
    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // We can also define the enum as a struct
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    
    #[derive(Debug)]
    enum IpAddrEnum {
        V4(String),
        V6(String),
    }

    let home = IpAddrEnum::V4(String::from("192.168.1.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    #[derive(Debug)]
    enum IpAddrEnum2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrEnum2::V4(192, 168, 1, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

}
