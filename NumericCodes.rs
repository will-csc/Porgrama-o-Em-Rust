use std::str::Chars;

fn main(){
    print_codes("William");
}
fn print_codes(s: &str) {
    let mut iter = s.chars();
    loop {
        match iter.next() {
            Some(c) => { println!("{}: {}", c, c as u32); },
            None => { break; },
        }
    }
}
