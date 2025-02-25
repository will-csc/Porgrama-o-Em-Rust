use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    println!("The '{}' value is: {}","Blue",scores.get("Blue"));
    println!("The '{}' value is: {}","Yellow",scores.get("Yellow"));
}

