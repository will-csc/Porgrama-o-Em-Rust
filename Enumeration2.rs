enum Continent {
    Africa,
    America,
    Asia,
    Europe,
    Oceania,
}
impl Continent {
    fn name(&self) -> &str {
        match *self {
            Continent::Africa => "Africa",
            Continent::America => "America",
            Continent::Asia => "Asia",
            Continent::Europe => "Europe",
            Continent::Oceania => "Oceania",
        }
    }
}
fn main(){
    print!("{}", Continent::Asia.name());
}