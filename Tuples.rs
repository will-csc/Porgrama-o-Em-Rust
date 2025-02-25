fn main(){
    let human: (String, i32, bool) = ("Alice".to_string(),30,false);
    println!("Human Tuple: {:?}\n", human);

    let decimals: (i32, i32, i32) = (10,20,30);
    println!("The decimals tuple: {:?}\n",decimals);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My mix tuple: {:?}", my_mix_tuple);
}
