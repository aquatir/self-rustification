fn main() {
    let separator = "****";
    println!("{separator}");
    println!("enums");
    enums();

    println!("{separator}");
    println!("option");
    option();

    println!("{separator}");
    println!("match");
    match_call();
}

fn option() {
    let some_num = Some(5);
    let some_str = Some("hello");
    let no_num: Option<i32> = None;

    dbg!(&some_num, &some_str, &no_num);

    let num_unwrapped = some_num.unwrap_or_default();
    let no_num_unwrapped = no_num.unwrap_or_default();

    dbg!(num_unwrapped, no_num_unwrapped);
}

fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    IpAddrKind::route(four);
    IpAddrKind::route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(&home);
    dbg!(&loopback);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn route(ip_kind: IpAddrKind) {
        println!("routing on type {ip_kind:?}")
    }
}

fn match_call() {}
