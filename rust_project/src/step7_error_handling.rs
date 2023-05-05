pub fn run() {
    let s: String = String::from("-123");
    match parse_positive_integer(&s) {
        Ok(result) => println!("parsed integer is {}", result), 
        Err(e) => println!("Error: {}", e)
    }
}

fn parse_positive_integer(s: &str) -> Result<u32, String> {
    match s.parse::<i32>() {
        Ok(num) => {
            if num > 0 {
                Ok(num as u32)
            }
            else {
                Err("The number should be positive.".to_string())
            }
        }
        Err(_) => Err("Failed to parse the string as an integer.".to_string())
    }
}