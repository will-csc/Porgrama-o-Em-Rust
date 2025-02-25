fn main(){
    let slice = &mut [3, 4, 5];
    {
        let iterator = slice.iter();
        for item_ref in iterator {
            *item_ref += 1; // It does generate an error
        }
    }
    print!("{:?}", slice);
}
