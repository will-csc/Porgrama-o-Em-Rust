fn main(){
    let mut v = vec![11,22,33];
    for _ in 0..5{
        let item: Option<i32> = v.pop();
        //Option<T> is used because it can have a null value
        //or not
        match item{
            Some(number) => print!("{}, ",number),
            None => print!("#, "),
        }
    }
}