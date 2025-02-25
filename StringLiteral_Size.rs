use std::mem::*;

fn main(){
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdè";
    
    print!("a: {} {} {}; \n",
        size_of_val(a),
        size_of_val(b),
        size_of_val(c));
    print!("&a: {} {} {}; \n",
        size_of_val(&a),
        size_of_val(&b),
        size_of_val(&c));
    print!("&&a: {} {} {}",
        size_of_val(&&a),
        size_of_val(&&b),
        size_of_val(&&c));
}