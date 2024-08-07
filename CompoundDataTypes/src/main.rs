#![allow(unused_variables)]
#![allow(dead_code)]
// arrays, tuples, slices and strings (slice string)

fn array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("numbers: {:?}", numbers);

    // let mix = [1, 2, 3, "four", true];   // error: expected integer, found `&str`

    let fruits: [&str; 5] = ["apple", "banana", "cherry", "date", "elderberry"];
    
    println!("fruits: {:?}", fruits);

    let data: [i32; 5] = [0; 5];  // [0, 0, 0, 0, 0]

    println!("data: {:?}", data);

    let first: i32 = numbers[0];
    let second: i32 = numbers[1];

    println!("first: {}, second: {}", first, second);

    // let length = 10;
    // let mut numbers: [i32; length] = [0; length]; // error: constant expression depends on a runtime value
}

fn tuple() {
    let human: (&str, i32, f64, bool, String) = ("Alice", 30, 170.0, true, "Hello".to_string());

    let name: &str = human.0;
    let age: i32 = human.1;
    let height: f64 = human.2;
    let is_adult: bool = human.3;
    // let message: String = human.4; // warn: partially moved value

    println!("Human Tuple: {:?}", human); // gives error when we define message variable before this line because human.4 is partially moved and no longer in tuple

    let message: String = human.4;

    println!("name: {}, age: {}, height: {}, is_adult: {}, message: {}", name, age, height, is_adult, message);

    let my_mix_tuple: (i32, &str, f64, bool, [i32; 5]) = (1, "two", 3.0, false, [1, 2, 3, 4, 5]);

    println!("my_mix_tuple: {:?}", my_mix_tuple);

    let (a, b, c, d, e) = my_mix_tuple;

    println!("a: {}, b: {}, c: {}, d: {}, e: {:?}", a, b, c, d, e);
}

fn slice() {
    // in arrays it declared as [i32; 5] but in slices it can be declared as &[i32] without size
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];

    println!("number_slices: {:?}", number_slices);

    let number_slices: &[i32] = &number_slices[1..4];

    println!("number_slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["ant", "bat", "cat", "dog", "elephant"];

    println!("animal_slices: {:?}", animal_slices);

    let book_slices: &[String] = &[
        "Alice in Wonderland".to_string(),
        "Brave New World".to_string(),
        "Charlie and the Chocolate Factory".to_string(),
        "Dune".to_string(),
        "Ender's Game".to_string()
    ];

    println!("book_slices: {:?}", book_slices);    
}

// Strings vs &str (String Slice)
fn strings_vs_string_slices() {
    // strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");

    println!("stone_cold: {}", stone_cold);

    stone_cold.push_str("Yeah!");

    println!("stone_cold: {}", stone_cold);

    // string slices [ fixed size, immutable, reference to a string ]
    let string: String = String::from("Hello, World!");

    let string_slice: &str = &string[7..12];

    println!("string_slice: {}", string_slice);
}

// println!("string: {}", string); // error: cannot find value `string` in this scope

fn main() {
    // array();
    // tuple();
    // slice();
    strings_vs_string_slices();
}
