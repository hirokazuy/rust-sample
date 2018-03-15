#[derive(Debug)]
struct IpV4Addr {
    addr: u32
}
#[derive(Debug)]
struct IpV6Addr {
    addr: String
}

impl IpV4Addr {
    fn from(v1: i8, v2: i8, v3: i8, v4: i8) -> IpV4Addr {
        IpV4Addr {
            addr: (v1 as u32) << 24
                | (v2 as u32) << 16
                | (v3 as u32) << 8
                | v4 as u32,
        }
    }
}
impl IpV6Addr {
    fn from(addr: String) -> IpV6Addr {
        IpV6Addr {
            addr
        }
    }
}

#[derive(Debug)]
enum IpAddress {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

fn main() {
    let addr = IpAddress::V4(IpV4Addr::from(127, 0, 0, 1));
    println!("addr is {:#?}", addr);
    let addr6 = IpAddress::V6(IpV6Addr::from(String::from("::1")));
    println!("addr is {:#?}", addr6);
}
