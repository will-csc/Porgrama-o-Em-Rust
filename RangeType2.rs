use std::ops::*;

fn main(){
    let range: Range<usize> = 3..8;
    println!("{:?}, {}, {}, {}",
        range, range.start, range.end, range.len());
}