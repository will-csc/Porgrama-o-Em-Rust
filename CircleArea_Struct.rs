const PI: f32 = 3.14159;

struct Circle{
    ray: u32,
}
fn main(){
    let circle1 = Circle{ray: 10};
    println!("\nThe circle's ray is: {}\nAnd his area is: {}",circle1.ray,area(&circle1));
}
fn area(circle: &Circle) -> f32{
        return (circle.ray * circle.ray) as f32 * PI;
}
