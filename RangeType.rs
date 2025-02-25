fn main(){
    let dozen = 0..12;
    // Dozen in this case is "Range<i32>" type
    
    println!("{:?}",dozen);
    for i in dozen { println!("{}", i); }
}