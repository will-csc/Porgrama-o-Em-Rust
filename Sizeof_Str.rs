use std::mem::size_of;

fn main(){
    let a: &str;
    fn f(a: &str){};
    print!("{}", size_of::<&str>());
}