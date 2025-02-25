fn main(){
    let email = String::from("william.cesar2016@outlook.com");
    let username = String::from("William C");
    let user1 = build_user(email,username);
    println!("The username is: {}",user1.username);
}

fn build_user(email: String, username: String) -> User{
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

