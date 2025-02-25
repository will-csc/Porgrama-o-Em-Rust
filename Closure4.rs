fn main(){
    let factor = 2;
    let multiply = |a| a * factor;
    
    print!("{}\n", multiply(13));
    
    let multiply_ref: &(Fn(i32) -> i32) = &multiply;
    
    print!("{} {} {} {} {}",
        (*multiply_ref)(10),
        multiply_ref(13),
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13));
}
