fn main(){
    let a: i8 = 5; // The correct type
    let b: i16 = 5; // Waste of 13 bits
    let c: i32 = 5; // Waste of 29 bits
    let d: i64 = 5; // Waste of 61 bits
    print!("{} {} {} {}", a, b, c, d);
}
