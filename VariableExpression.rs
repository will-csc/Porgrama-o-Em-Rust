fn main() {
    let x = 5;
    //let y = (let x = 3; x + 7) it does not work
    let y = { // Expression (it does work)
        let x = 3;
        x + 7
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}