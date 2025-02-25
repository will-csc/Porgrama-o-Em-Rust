fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    return largest;
}
fn main(){
    let number_list = vec![18,19,21,52,50,86];
    let n: i32 = largest(&number_list);
    println!("The largest number is: {}",n);
}
