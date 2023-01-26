fn main() {
    enum IpAddrKind {
        V4(String),
        V6(String),
    }
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home2 = IpAddr::V4(127, 0, 0, 1);
    let loopback2 = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

// We can also define methods on enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn route(ip_kind: IpAddrKind) {}
