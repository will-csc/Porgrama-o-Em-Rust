fn main(){
    let array = [12, 13, 14];
    let tuple = (12, 13, 14);
    let i = 0;
    println!("{}", array[i]);
    //print!("{}", tuple.i); //Compiler error
    println!("{}", tuple.0);
}
