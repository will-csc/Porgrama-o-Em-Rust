fn main(){
    let r1 = divide(8., 2.);
    let r2 = divide(8., 0.);
    println!("{} {}", r1.is_ok(), r2.is_ok());
    println!("{} {}", r1.is_err(), r2.is_err());
    println!("{}", r1.unwrap());
    // "unwrap()" function returns a the value if it is "Ok"
    // otherwise it panics, it will happen in the next line
    println!("{}", r2.unwrap());
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