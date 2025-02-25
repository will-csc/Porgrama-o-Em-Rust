fn main(){
    let x = 10.;
    print_double(x); //Primitive types does not implement ownership
                     //in Rust
    print!("\n{}",x);
}
fn print_double(x: f64){
    print!("{}",x);
}

