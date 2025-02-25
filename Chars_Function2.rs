use std::str::Chars;

fn main(){
    print_nth_char("€èe", 3);
}
fn print_nth_char(s: &str, mut n: u32){
    let mut iter: Chars = s.chars();
    print!("{:?}",iter);
    // 'Chars()' function returns an array of characters
    // output: Chars(['€','è','e'])
}
