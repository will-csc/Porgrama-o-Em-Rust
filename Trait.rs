fn main(){
    fn double(x: i32) -> i32 {
        x * 2
    }
    trait canBeDouble{
        fn double(self) -> Self;
    }
    impl canBeDouble for i32 {
        fn double(self) -> Self{
            self * 2
        }
    }
    print!("{}", 7i32.double());
}