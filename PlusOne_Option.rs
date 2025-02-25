fn main() {
    let x: Option<i32> = None;
    plus_one(x);
}
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => {
            println!("The value inside is {}",1);
            Some(1)
        },
        Some(i) => {
            println!("The value inside is now {}",i+1);
            Some(i + 1)
        }
    }
}