use std::mem::size_of;
use std::mem::size_of_val;

fn main(){
    // Here we can see the size of one's variables
    // in Bytes
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}