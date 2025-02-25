use std::io::stdin;

fn main(){
    let mut text = format!("First: ");
    let inp = stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("Second: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {} bytes", text, text.len());
}