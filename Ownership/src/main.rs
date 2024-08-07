#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let s1 = String::from("Rust");
    // let len = calculate_length(s1); // s1 is moved to calculate_length
    // println!("The length of {} is {}.", s1, len); // error because s1 is moved
    let len = calculate_length_borrow(&s1);

    println!("The length of {} is {}.", s1, len); // no error because s1 is borrowed

    let s2 = s1;
    // println!("The length of {} is {}.", s1, len); // error because s1 is moved
}

// fn print_lost(s: &str) {
//     println!("{}", &s1); // error because s1 is not in scope
// }

fn calculate_length(s: String) -> usize {
    s.len()
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}