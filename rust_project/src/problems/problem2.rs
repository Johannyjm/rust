pub fn run() {
    let rect2: Rectangle = Rectangle {
        width: 30, 
        height: 40, 
    };

    println!("rect2's area is {}", rect2.area());
}

struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}