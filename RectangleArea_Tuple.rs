fn main(){
    let rect1 = (30,50);    
    println!("The area with {} width and {} height is: {}",rect1.0,rect1.1,area(rect1));
}
fn area(dimensions: (u32,u32)) -> u32{
    return dimensions.0 * dimensions.1;
}
