fn main(){
    show_divide(8.,2.);
    show_divide(8.,0.);
}
fn divide(numerator: f64, denominator: f64) -> Result<f64,String> {
    if denominator == 0.{
        Err(format!("Divide by zero"))
    }else{
        Ok(numerator / denominator)
    }
}
fn show_divide(num: f64, den: f64){
    match divide(num, den){
        Ok(res) => println!("{} / {} = {}", num, den, res),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}