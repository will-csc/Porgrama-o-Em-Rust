fn main(){
    let some_u8_value = 7;
    match some_u8_value{
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //Every other possibility
    }
}