fn main(){
    let x: f64 = 3.145926535;
    let double_x: f64 = double(x);
    print!("pi * 2 = {}",double_x);
}
fn double(x: f64) -> f64{
    return x * 2.0;
}
