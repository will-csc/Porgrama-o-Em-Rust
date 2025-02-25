use std::io::Write;
use std::fs::File;

fn main(){
    let mut file = File::create("data.txt").unwrap();
    file.write_all("eè€".as_bytes()).unwrap();
}
