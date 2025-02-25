fn main(){
    let mut s = "".to_string();
    for _ in 0..10 {
        println!("{:?} {} {}",
        s.as_ptr(), s.capacity(), s.len());
        // "as_prt()" function returns the memory address of one
        // variable
        s.push('a');
    }
    println!("{:?} {} {}\n{}",
    s.as_ptr(), s.capacity(), s.len(), s);
}