fn main(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s2 + &s1; // note s1 has been moved here and can no longer be used
    // &s1 + &s2 will cause an error, slices cannot be added
    
    println!("The string 's3' after sum: {}",s3);
    println!("Ownership rules apllying: {}",&s1);
}
