enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost_v4 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let localhost_v6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("00:00:00:00:00:00"),
    };

    println!("v4 address: {}", localhost_v4.address);

    println!("v6 address: {}", localhost_v6.address);
}
