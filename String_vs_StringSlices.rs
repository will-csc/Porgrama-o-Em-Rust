fn main(){
    //Strings vs String Slices (&str)
    
    //String [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}",stone_cold);
    stone_cold.push_str("Yeah!");

    //&str (String Slice)
    let williaminput: String = String::from("Hello, World!");
    let slice: &str = &williaminput;
    println!("Slice Value: {}",slice);

    //Defining an size
    let slice2: &str = &williaminput[0..5];
    println!("Second Slice Value: {}",slice2);
}
