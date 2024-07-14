use std::net::IpAddr;


fn main() {
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }


    // let _home = IpAddr::V4(String::from("127.0.0.1"));
    // let _loopback = IpAddr::V6(String::from("::1"));


    // ENhanced enums


    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));


    // struct  IpAddr{
    //     kind: IpAddrKind,
    //     address: String
    // }

    // let _four = IpAddrKind::V4;
    // let _six = IpAddrKind::V6;

    // let _home: IpAddr = IpAddr {
    //      kind: _four,
    //      address: String::from("127.0.0.1") 
    //     };

    // let _loopback: IpAddr = IpAddr { kind: _six, address: String::from("::1") };
   

   
}