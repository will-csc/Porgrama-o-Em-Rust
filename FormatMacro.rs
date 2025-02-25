fn main(){
    let s1 = String::from("William");
    let s2 = String::from("Cesar");
    let s3 = String::from("Carvalho");

    let s = format!("{}-{}-{}",s1,s2,s3); // "format!()" macro returns a string
    println!("The string using \"format!\": {}",s);
}
