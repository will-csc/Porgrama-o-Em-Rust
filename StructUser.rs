fn main(){
    let mut user1 = User{
        email: String::from("williamcscarvalho@protonme.com"),
        username: String::from("William"),
        active: true,
        sign_in_count: 1,
    };

    println!("The e-mail before changing is: {}",user1.email);
    user1.email = String::from("william.cesarbds2016@gmail.com");
    println!("The e-mail after changing is: {}",user1.email);
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

