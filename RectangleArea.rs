fn main(){
    let width: u32 = 30;
    let height: u32 = 50;

    println!("The rectangle with {} width and {} height is: {}",width,height,calculate_area(width,height));
}
fn calculate_area(width: u32,height: u32) -> u32{
    return width * height;
}
