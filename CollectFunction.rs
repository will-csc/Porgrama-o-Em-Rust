fn main(){
    let arr = [36, 1, 15, 9, 4];
    let v = arr.iter().collect::<Vec<&u32>>();
    print!("{:?}", v);
}
