fn main(){
    let frase = String::from("William vai vencer na vida!");
    println!("Ownership of 'frase': {}",frase);
    
    let frase_copiada = &frase;
    println!("Borrowing: {}",frase_copiada)
}