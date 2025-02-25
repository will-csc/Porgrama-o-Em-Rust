struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main(){
    let rect1: Rectangle = Rectangle{width:5,height:4};
    let rect2: Rectangle = Rectangle{width:4,height:4};
    
    println!("area: {}",rect1.area());
    println!("can hold: {}",rect1.can_hold(&rect2));
}