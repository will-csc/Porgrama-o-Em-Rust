fn main(){
    #[derive(Debug)]
    enum Result{Success(f64), Failure(u16, char), Uncertainty,}

    let outcome = Result::Success(23.67);
    println!("{:?}",outcome);
    let outcome = Result::Failure(1200,'X');
    println!("{:?}",outcome);

    match outcome{
        Result::Success(value) => println!("Result: {}",value),
        Result::Failure(error_code, module) => println!("Error n. {} in module {}",error_code,module),
        Result::Uncertainty => {},
    }
}
