fn main(){
    let arr = [45,8,-2,6];
    match arr.iter().min(){
        Some(n) => print!("{} ",n),
        _ => (),
    }
    match arr.iter().max(){
        Some(n) => print!("{} ",n),
        _ => (),
    }
}
