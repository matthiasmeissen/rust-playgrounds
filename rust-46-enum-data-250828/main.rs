
#[derive (Debug)]
enum IpAddrKind {
    V4,
    V6
}

fn main() {
    let home = IpAddrKind::V4;
    let loopback = IpAddrKind::V6;

    println!("Home Ip is: {:?}", home);
    println!("Loopback Ip is: {:?}", loopback);
}
