fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {

}

enum IpAddrKind {
    V4,
    V6
}