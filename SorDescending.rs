use std::cmp::Ordering;

fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    // Sorting an array in descending order
    arr.sort_by(desc);
    println!("{:?}", arr);  // Usamos `println!` para imprimir a array
}

fn desc(a: &i32, b: &i32) -> Ordering {
    if a < b { 
        Ordering::Greater 
    } else if a > b { 
        Ordering::Less 
    } else { 
        Ordering::Equal 
    }
}
