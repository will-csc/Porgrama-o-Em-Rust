fn main(){
    let mut dyn_str = "Hello".to_string();
    
    // "push_str()" adds an string into the end of one's
    // string
    dyn_str.push_str(", ");
    dyn_str.push_str("world");
    dyn_str.push_str("!");
    print!("{}", dyn_str);
}