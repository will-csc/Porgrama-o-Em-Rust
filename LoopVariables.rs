fn main(){
    let index = 8;
    for index in 0..4{ //You can use the same name
                       //The other variable will be
                       //shadowed
        print!("{} ",index);
    }
    print!(":{}",index);
}

