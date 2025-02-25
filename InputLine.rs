use std::io;

fn main(){
    let mut line = String::new();
    println!("{:?}",io::stdin().read_line(&mut line));
    println!("[{}]",line);
}