fn main(){
    let tup = (500,6.4,1);
    println!("The value of 'tup' is: {:?}",tup);

    let (_x,y,_z) = tup;
    println!("The value of y ('tup' position 2) is: {}",y);
}
