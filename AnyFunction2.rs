fn main(){
    print!("{}",
          [45,8,-1,6].iter().any(|n| *n < 0))
}
