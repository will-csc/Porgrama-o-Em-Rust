fn main(){
    f();
    fn f(){println!("It is working!")}
    shadowing();
}
fn f(){
    println!("It is working in another block");
}
fn shadowing(){
    f();
}
