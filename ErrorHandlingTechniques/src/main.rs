#![allow(unused_variables)]
#![allow(dead_code)]

enum Option<T> {
    Some(T), // Represents the presence of a value
    None,   // Represents the absence of a value
}

enum Result<T, E> {
    Ok(T),    // Represents a success with a value
    Err(E),   // Represents an error with a value
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        Option::None
    } else {
        Option::Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Result::Err("Cannot divide by zero".to_string())
    } else {
        Result::Ok(numerator / denominator)
    }
}

fn main() {
    let result = divide(2.0, 3.0);
    match result {
        Option::Some(value) => println!("Result: {:.2}", value),
        Option::None => println!("Cannot divide by zero"),
    }

    let result = divide(2.0, 0.0);
    match result {
        Option::Some(value) => println!("Result: {:.2}", value),
        Option::None => println!("Cannot divide by zero"),
    }

    let result = divide_result(2.0, 3.0);
    match result {
        Result::Ok(value) => println!("Result: {:.2}", value),
        Result::Err(message) => println!("{}", message),
    }

    let result = divide_result(2.0, 0.0);
    match result {
        Result::Ok(value) => println!("Result: {:.2}", value),
        Result::Err(message) => println!("{}", message),
    }
    
}
