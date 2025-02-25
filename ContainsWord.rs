fn main(){
    let s = "Hello, world!";
    let ch = 'w';
    let mut contains = false;
    
    for c in s.chars() {
        if c == ch {
            contains = true;
            break;
        }
    }
    
    print!("\"{}\" {} '{}'.",
            s,
        if contains {
        "contains"
        } else {
        "does not contain"
        },
        ch);
}
