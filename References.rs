fn main(){
    let a = 10;
    let b = &a; //Using references
    print!("{} + {} = {}",a,*b,a + *b);
}