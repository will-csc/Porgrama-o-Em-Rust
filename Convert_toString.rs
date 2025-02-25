fn main(){
    let data = "inital contents";
    println!("Data without converting to string: {}",data);
    
    let s = data.to_string();
    println!("Data converting to string: {}",data);
    
    let s = "initial contents".to_string();
    println!("Data converted directly to string: {}",s);
}