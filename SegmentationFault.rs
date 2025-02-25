fn main(){
    recursive_func(N_ARRAY);
    const SIZE: usize = 100_000;
    const N_ARRAY: usize = 1_000_000;
    fn create_array() -> [u8; SIZE] { [0u8; SIZE] }
    fn recursive_func(n: usize) {
        let a = create_array();
        println!("{} {}", N_ARRAY - n + 1, a[0]);
        if n > 1 { recursive_func(n - 1) }
    }
    // It will crash your system, it is trying to allocate
    // more than 100gb in the stack
}
