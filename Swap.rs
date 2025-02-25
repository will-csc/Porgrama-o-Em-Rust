use std::mem::swap;

fn main(){
    let mut a = String::from('A');
    let mut b = String::from('B');
    print!("{}, {};\n", a, b);
    swap(&mut a,&mut b);
    print!("{}, {}", a, b);
}