fn main(){
    let mut v = vec![10,20,30,40,50];
    for i in 0..v.len(){
        v[i] *= 10;
    }
    for _ in 0..v.len() {
        print!("{}, ", v.pop().unwrap())
    }
}