fn main(){
    // let _r2 = 3..5_000_000_000;
    // The statement above generates an error, because
    // the generic type would be i32, and the end range
    // is beyond the scope
    
    let _r2 = 3..5_000_000_000 as u64;
}
