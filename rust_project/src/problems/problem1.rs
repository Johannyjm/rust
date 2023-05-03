pub fn run() {
    let circle1: Circle = Circle{
        radius: 5.0, 
    };

    println!("circle1's area is {}", circle1.area());
    println!("circle1's diameter is {}", circle1.diameter());
}

struct Circle {
    radius: f64
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
}