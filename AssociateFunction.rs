struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}

fn main(){
    let rect1 = Rectangle::square(5);
    println!("The rectangle size is: {}",rect1.width)
}