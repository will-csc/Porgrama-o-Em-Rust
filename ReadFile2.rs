use std::io::Read;
use std::fs::File;
use std::io::Write;

fn main(){
    let mut file = File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
