use std::cmp::Ordering;

fn main(){
    let mut arr = [1,1,2,3,5,13,8,21];
    // If the two elements are equal the closures "|" returns
    // the array equally, else, it does the ordering
    
    let desc = |a: &i32, b: &i32| -> Ordering{
        if a < b { Ordering::Less}
        else if a > b { Ordering::Greater }
        else { Ordering::Equal }
    };
    
    arr.sort_by(desc);
    print!("{:?}",arr);
}
