fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("division by zero")
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => {
            println!("Result: {}", result);
        }
        
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
