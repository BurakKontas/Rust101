fn main() {
    let x = 5;
    let y = 10;

    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // error: expected type `i32`, found type `&str`

    println!("The value of number is: {}", number);
}
