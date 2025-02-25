fn main(){
    let x: (i32,f64,u8) = (500,6.4,1);

    println!("The value of 'x' in position 1 is: {}",x.0);

    let one = x.2;
    println!("The value of 'x' in position 3 is: {}",one);
}
