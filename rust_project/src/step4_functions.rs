pub fn run() {
    greeting("Hello", "John");

    let sum = add(10, 20);

    println!("10 + 20 = {}", sum);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}