fn main(){
    let i = 8;
    let j = 8_000_000_000; //Compile Error, number out of range
                          //For an 'i32' range
    print!("{} {}", i, j);
}
