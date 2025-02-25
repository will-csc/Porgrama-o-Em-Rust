fn main(){
    let name: &str = "william";
    
    // The 'nth()' function is used to get a character
    // in a given index
    let c = name.chars().nth(0);
    print!("{:?}",c);
}
