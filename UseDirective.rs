use std::mem;

fn main(){
    print!("{} ",mem::size_of::<i32>());
    print!("{} ",mem::size_of_val(&12u8));
}