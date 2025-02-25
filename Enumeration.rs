enum Message{
    Quit, //No data associated
    Move{x: i32, y: i32}, //Struct
    Write(String), //String
    ChangeColor(i32,i32,i32), //i32 values
}
impl Message{
    fn call(&self){
        println!("It's working OK")
    }
}
fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();
}
