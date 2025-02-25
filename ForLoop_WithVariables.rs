fn main(){
    let mut limit = 4;
    for i in 1..limit{
        limit -= 1;
        print!("{} ",i);
    }
    print!(":{}", limit);
}
