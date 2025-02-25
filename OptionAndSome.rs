fn main(){
    let five = Some(5);
    let six = plus_one(five);
    println!("The sum result is: {:?}", six);
    
    let none = plus_one(None);
    println!("The sum result is: {:?}", none);
}
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}