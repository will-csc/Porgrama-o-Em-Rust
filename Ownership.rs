fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is '{}'.\n",s1,len);
    
    let frase: &str = "william c s de carvalho";
    println!("The name of the author is: {}",frase);
}
fn calculate_length(s: &String) -> usize{
    s.len()
}