fn main(){
    let mut s1 = String::from("foo");
    //let s2 = "bar"; //rules of ownership does not aplly
    let s2 = String::from("bar"); //rules of ownership does aplly
    s1.push_str(&s2);
    println!("s2 is {}", s2);
}
