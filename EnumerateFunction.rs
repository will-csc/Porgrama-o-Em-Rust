fn main(){
    let arr = ['a', 'b', 'c'];
    // 'enumerate' function works to get the index and also the
    // the value assigned in it
    for (i, ch) in arr.iter().enumerate() {
        print!("{} {}, ", i, *ch);
    }
}
