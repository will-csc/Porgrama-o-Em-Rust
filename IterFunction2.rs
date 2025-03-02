fn main(){
    // 'Iter()' function is used to make something
    // iterable
    for item_ref in (&[11u8, 22, 33]).iter() {
        // *item_ref += 1;
        print!("{} ", *item_ref);
    }
    
    for item_ref in [44, 55, 66].iter() {
        // *item_ref += 1;
        print!("{} ", *item_ref);
    }
    for item_ref in vec!['a', 'b', 'c'].iter() {
        // *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        print!("{} ", *item_ref);
    }
}
