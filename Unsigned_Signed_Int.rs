fn main(){
    let x: u8 = 1; // Can only be positive
    let y: i8 = -1; // Can be both positive and negative
    // Each signed can store number from -(2^n-1) to 2^n-1
    // Each unsigned can store numbers up to 2^n-1

    println!("The unsigned integer has the value of: {}",x);
    println!("The signed integer has the value of: {}",y);
}


