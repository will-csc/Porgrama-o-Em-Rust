fn main(){
    f2();
}
fn f1(){
    println!("-------------");
}
fn f2(){
    for _i in 0..3{
        f1();
    }
}
