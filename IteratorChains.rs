fn main(){
    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for n in arr
        .iter()
        .filter(|x| **x > 0)
        .map(|x| *x * 2)
        {
            v.push(n);
        }
    print!("{:?}", v);
}