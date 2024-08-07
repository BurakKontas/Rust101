#![allow(unused_variables)]
#![allow(dead_code)]
 
// an function / variables should be written in snake case
// snake case: hello_world
// kebab case: hello-world

// entry point
fn main() {
    // hello_world();
    // hello_world_with_name("John Doe");
    // hello_world_with_string("John Doe".to_string());

    // let human_id: String = human_id("John Doe", 25, 180.);
    // println!("{}", human_id);

    // expression();

    // let sum: i32 = add(10, 20);
    // println!("sum: {}", sum);

    // lambda();

    let weight: f32 = 82.5;
    let height: f32 = 1.82;

    let bmi: f32 = calculate_bmi(weight, height);
    println!("BMI: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, world!");
}

fn hello_world_with_name(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_world_with_string(name: String) {
    println!("Hello, {}!", &name[0..2]);
}

fn human_id(name: &str, age: u8, height: f32) -> String {
    format!("{}-{}-{}", name, age, height) // return is optional
}

// Expressions and Statements
// Statements: Anything that does not returns a value.
// Expressions: Anything that returns a value.

// const X : i32 = {
//     let y = 10;
//     y + 5
// };

fn expression() {
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("{}", x);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn lambda() {
    let lambda_add = |x: i32, y: i32| x + y;

    let lambda_sum: i32 = lambda_add(11, 21);

    println!("lambda_sum: {}", lambda_sum);
}

fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}