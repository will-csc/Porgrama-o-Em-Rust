fn main(){
    let factor = 2;
    let multiply = |a|a * factor;
    // Inside the "| |" are the arguments, after it is the
    // return result
    
    for i in 1..11{
        println!("{}",multiply(i));
    }
}
