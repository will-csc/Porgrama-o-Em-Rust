#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self) -> u32{
      return self.width * self.height;
    }
}
fn main(){
    let rect1 = Rectangle{width: 30, height: 50};
    println!("The rectangle is: {:?}",rect1);
    println!("And his area is: {}",rect1.area());
}
