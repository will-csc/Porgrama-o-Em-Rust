fn main(){
    println!("Number {} in binary: {}", 10, binary(10));
}
fn binary(n: i32) -> String{
    if n == 1{
        return "0".to_string();
    }
    
    let mut result = String::new();
    let mut num = n;

    while num > 0 {
        result.insert(0, if num % 2 == 0 { '0' } else { '1' });
        num /= 2;
    }

    return result;
}