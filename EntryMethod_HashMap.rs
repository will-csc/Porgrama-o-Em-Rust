use std::collections::HashMap;

fn main(){
    let mut ages = HashMap::new();

    ages.insert(String::from("William"),21);
    ages.entry(String::from("Eliane")).or_insert(53);

    println!("{:?}",ages);
}
