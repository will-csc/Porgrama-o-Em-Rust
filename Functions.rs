// an function/variables should be written in snake
// snake case: hello_world
// rebab case: hello-world

fn main(){
    hello_world();
    tell_height(1.80);
    you("William",21);
    let _x: i32 = {
        let price: i32 = 5;
        let qtd: i32 = 10;
        price * qtd
    };
    println!("Result is: {}",_x);

    let result_add: i32 = add(10,5);
    println!("The result of addition is: {}",result_add);

    let weight: f64 = 90.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight,height);
    println!("The current bmi with 90 weight and 1.82 height: {}",bmi);
}
//Hoisiting - can call function anywhere in your code
fn hello_world(){
    println!("Hello world!");
}
// you can insert input values
fn tell_height(height: f32){
    println!("My height is: {}", height);
}

//you can insert more than one value
fn you(name: &str, age: i32){
    println!("Your name is: {} | and your age is: {}",name,age);
}

// Functions returning values
fn add(a: i32, b: i32) -> i32{
    return a + b;
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    return weight_kg / (height_m * height_m)
}

// Expression and Statements
// Expression: Anything that returns a value
// Statement: Anything that does not return a value
