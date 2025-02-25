fn main(){
    let condition = true;

    let number = if condition{
                        5
                }else{
                    "six" //this will return an error, it must be the same type
                };
    println!("The value of number is: {}",number);
}
