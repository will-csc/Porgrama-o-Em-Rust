use std::cmp::Ordering;

fn main(){
    let mut arr = [1,1,2,3,5,8,13,21];
    
    let desc = |a: &i32, b: &i32| -> Ordering{
        if a < b { Ordering::Less}
        else if a > b { Ordering::Greater }
        else { Ordering::Equal }
    };
    
    arr.sort_by(desc);
    print!("{:?}",arr);
}
