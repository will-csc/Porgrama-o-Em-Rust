fn main(){
    let mut x = [4.;100]; //if we wanted to inicialized it with
                           //other values like 3.14 we could
    x[50] = 5.6;
    print!("{} | {}",x[50],x[99]);
}

