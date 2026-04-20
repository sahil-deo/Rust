
// Part 1

// Using enum to only store the kind of IP. 
enum IpAddrKind {
    V4, 
    V6
}

// Using a struct with the kind enum as a varient. 
// addr to store the actual address. 
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

//Part 2

// Using the enum itself to store the value of the address as well. 
enum Ip {
    V4(String),
    V6(String)
}

fn checkEnum(kind: IpAddrKind) -> String {
    match (kind) {
        IpAddrKind::V4 => "V4".to_string(),
        IpAddrKind::V6 => "V6".to_string(),
    }
}

fn main(){
    

    // using the struct to create instances. 
    let home: IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        addr: String::from("192.168.1.1")
    };

    let work: IpAddr = IpAddr{
        kind: IpAddrKind::V6,
        addr: String::from("::1")
    };


    // using enum to store addresses. 
    let e_home: Ip = Ip::V4(String::from("192.168.1.1"));
    let e_work: Ip = Ip::V6(String::from("::1"));

    // printing values
    println!("Home {}", checkEnum(home.kind));
    println!("Work {}", checkEnum(work.kind));


}