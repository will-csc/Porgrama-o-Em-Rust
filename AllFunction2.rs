fn main(){
    print!("{} ", [45, 8, 2, 6].iter()
        .all(|n: &i32| -> bool { *n > 0 }));
    // The "all" function works like the "any", but the conditions
    // must be applied to all elements inside
    print!("{} ", [45, 8, -2, 6].iter()
        .all(|n: &i32| -> bool { *n > 0 }));
}
