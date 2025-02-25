fn main(){
    let mut x = vec!["This","is"];
    println!("Length: {}", x.len());
    
    x.push("a");
    println!("The length after 'push': {}",x.len());

    x.push("sentence");
    println!("The length after 'push' a word: {}",x.len());

    x[0] = "That";
    println!("Printing with a for loop:\n");
    for i in 0..x.len(){
        print!(" {}", x[i]);
    }
}

