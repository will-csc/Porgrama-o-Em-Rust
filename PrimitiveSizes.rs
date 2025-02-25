use std::mem::*;

fn main(){
    print!("{} {} {} {} {} {} {} {} {} {} {} {}",
        size_of::<i8>(),
        size_of::<u8>(),
        size_of::<i16>(),
        size_of::<u16>(),
        size_of::<i32>(),
        size_of::<u32>(),
        size_of::<i64>(),
        size_of::<u64>(),
        size_of::<f32>(),
        size_of::<f64>(),
        size_of::<bool>(),
        size_of::<char>());
}