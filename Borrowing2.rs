fn main(){
    let v1 = vec![11, 22, 33];
    let v2 = v1;
    
    // Borrows occurs in "v2" variable, so "v1" is dropped
    print!("{}", v2.len());
}