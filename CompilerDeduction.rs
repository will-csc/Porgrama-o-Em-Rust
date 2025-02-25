fn main(){
    let x = 0; // It first assigns usize to this variable
    let _j: u16 = x; // After reading this the compiler realizes the
    // 'x' variable is used to inicialize '_j', so it should be of
    // the same type
}
