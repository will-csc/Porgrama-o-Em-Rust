use std::fs::File;

fn main(){
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // "Expect" command is used to display a message if
    // "File::open" cannot open a file
}

