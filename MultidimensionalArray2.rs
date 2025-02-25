fn main(){
    let x = [[[[0;4];6];8];15];
    print!("{}, {}, {}, {}.",
            x.len(),x[0].len(),x[0][0].len(),x[0][0][0].len());
    println!("'x' values: {:?}",x);
}
