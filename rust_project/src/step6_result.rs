pub fn run() {
    let a = 10.0;
    let b = 7.0;

    match divide(a, b) {
        Ok(result) => println!("{} / {} = {}", a, b, result), 
        Err(e) => println!("Error: {}", e)
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero.".to_string())
    }
    else {
        Ok(a / b)
    }
}