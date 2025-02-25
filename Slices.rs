fn main(){
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}\n",number_slices);

    let people:&[&str] = &["William","Tamiles","Julio","Eliane"];
    println!("The people of my life are: {:?}",people);

    let book_slices: &[&String] = &[&"Harry Potter".to_string(),&"Animal Farm".to_string(),&"Maus".to_string()];
    println!("Book slice: {:?}", book_slices);
}

