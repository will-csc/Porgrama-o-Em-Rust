fn main(){
    let x = swap(3i16, 4u16);
    let y = swap(5f32, true);
    print!("{:?} {:?}", x, y);
}
fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
