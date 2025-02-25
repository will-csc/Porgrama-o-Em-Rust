fn main(){
    let a: (i32,i32) = divide(30,10);
    println!("Quocient: {}\nRest: {}",a.0,a.1);
}
fn divide(divid: i32, divis: i32) -> (i32,i32){
    return (divid / divis, divid % divis);
}