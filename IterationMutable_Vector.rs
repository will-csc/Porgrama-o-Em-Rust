fn main(){
    let mut v = vec![100,32,57];
    println!("The vector value before change: {:?}",v);

    for i in &mut v{
        *i += 50;
    }

    println!("The vector value after +50 change: {:?}",v);
}
    

