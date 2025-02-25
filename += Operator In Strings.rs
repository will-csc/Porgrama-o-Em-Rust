fn main(){
    let mut dyn_str = "Hello".to_string();
    
    // The "+=" operator is used to concactenate strings
    // just like the "push_str()" method
    dyn_str += ", ";
    dyn_str += "world";
    dyn_str += "!";
    print!("{}", dyn_str);
}