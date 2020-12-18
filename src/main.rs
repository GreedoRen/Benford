enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    adress: String,
}

fn route(ip_kind: IpAddrKind) {

}


fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    let home = IpAddr{
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adress: String::from("::1"),
    };
}
