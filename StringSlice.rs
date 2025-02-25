fn main(){
    let name = String::from("William");
    let s = &name[0..4];

    println!("The string slice: {}",s);

    let text: &str = "Teste de string";
    let s2 = &text[2..6];
    
    println!("The string slice 2: {}",s2);
}

