enum TrafficLight{
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red,Yellow};

fn main(){
    let red = Red; //No need for "TrafficLight::" Syntax
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

