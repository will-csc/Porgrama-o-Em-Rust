fn main(){
    const TWO: f64 = 2.; // Here we have to declare it constant
                        // and his type
    fn print_double(x: f64){
        print!("{}", x * TWO);
    }
    print_double(17.2);
}
