use std::mem::size_of;
use std::mem::size_of_val;

fn main(){
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}