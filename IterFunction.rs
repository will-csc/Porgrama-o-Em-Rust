fn main(){
    // 'Iter()' function is used to make something
    // iterable
    for item_ref in (&[11u8, 22, 33]).iter() {
        // *item_ref += 1;
        print!("{} ", *item_ref);
    }
}
