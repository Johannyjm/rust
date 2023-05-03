pub fn run() {
    greeting("Hello", "John");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}