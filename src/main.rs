enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    adress: String,
}

fn route(ip_kind: IpAddrKind) {

}


fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
