fn main(){
    let mut i = 0;
    while i < 50{
        i += 1;
        if i % 3 == 0 {continue;}
        if i * i > 400 {break; }
        print!("{} ",i * i);
    }
}
