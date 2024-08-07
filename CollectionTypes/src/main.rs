#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    // vectors();
    hash_maps();
}

fn vectors() {
        let mut vec: Vec<i32> = Vec::new();
    let vec_macro: Vec<i32> = vec![1, 2, 3];
    let vec_zeros: Vec<i32> = vec![0; 10]; // 10 zeros

    vec.push(5);
    vec.push(6);
    vec.push(7);

    // vec_macro.push(10); // error: cannot mutate immutable variable `vec_macro`

    println!("{:?}", vec);
    println!("{:?}", vec_macro);
    println!("{:?}", vec_zeros);

    let third: &i32 = &vec[2]; // Direct indexing
    println!("The third element is {}", third);

    match vec.get(2) { // Safe indexing
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match vec.get(100) {
        Some(hundredth) => println!("The hundredth element is {}", third),
        None => println!("There is no 100th element."),
    }
}

fn hash_maps() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 100);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(-1);

    println!("{:?}", scores);
    println!("{:?}", score);

    let score = scores.get("Red").copied().unwrap_or(-1);
    println!("{:?}", score);

    for(key, value) in &scores {
        println!("{}: {}", key, value);
    }
}