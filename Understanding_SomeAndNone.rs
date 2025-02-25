fn main() {
    let x = Some(5);   // Não precisa ser Option::Some(5)
    let y: Option<u16> = None; // Também simplificado
    println!("{:?}", y);
}