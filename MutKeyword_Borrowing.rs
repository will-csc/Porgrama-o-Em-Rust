fn main(){
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    
    let mut p: &mut i32 = &mut a;
    
    print!("{}",*p);
    *p += 1;
    
    print!("\n{}", *p);
    p = &mut b;
    
    print!("\n{}", *p);
    *p += 1;
    
    print!("\n\na: {}\nb: {}\np: {}", a,*p,*p); //Cannot use 'b' after
                                         //Borrows it's value
}
