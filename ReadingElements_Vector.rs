fn main(){
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    let fourth: Option<&i32> = v.get(3);

    println!("The value for number 2 is: {:?}",third);
    println!("The value for number 3 is: {:?}",fourth);
}
