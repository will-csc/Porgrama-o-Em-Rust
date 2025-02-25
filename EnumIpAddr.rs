
fn main(){
    #[derive(Debug)]
    enum IpAddr{
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("The home address is: {:?}\nThe loopback address is: {:?}",home,loopback);
}
