fn main(){
    #[derive(Debug)]
    enum CardialPoint{ North, South, West, East}
    let direction = CardialPoint::North;
    match direction{
        CardialPoint::North => println!("Cordenates: {:?}",direction),
        _ => println!("Any other alternatives"),
    }
}

