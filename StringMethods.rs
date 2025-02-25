fn main(){
    let mut a: String = "Xy".to_string(); // "Xy"
    a.remove(0); // remove at the index '0'
    a.insert(0, 'H'); // Insert the character 'H' at the '0'
                      // position
    a.pop(); // Removes the last character, and returns it
    let s = a.pop().unwrap();
    
    a.push('i'); // Adds the 'i' character
    
    print!("a: {}\n", a);
    print!("s: {}\n", s);
}