fn main(){
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The string length is: {}",len)
}
fn calculate_length(s: String) -> usize { // s is a reference to a String
    s.len()
}