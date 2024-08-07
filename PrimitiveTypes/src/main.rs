#![allow(unused_variables)]
#![allow(dead_code)]

// primitive types 
// int, float, bool, char

// Integers
// i8, i16, i32, i64, i128, isize: signed
// u8, u16, u32, u64, u128, usize: unsigned

fn integer() {
    let x: i32 = -42;
    let y: u32 = 42;

    println!("x: {}, y: {}", x, y);
    println!("Size of x is {} bytes", std::mem::size_of_val(&x));
    println!("Size of y is {} bytes", std::mem::size_of_val(&y));

    println!("Max i32: {}", std::i32::MAX);
    println!("Min i32: {}", std::i32::MIN);

    println!("Max u32: {}", std::u32::MAX);
    println!("Min u32: {}", std::u32::MIN);
}

fn float() {
    let pi: f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;
    let e: f32 = 2.718281828459045235360287471352662497757247093699959574966967627724076630353;

    println!("pi: {}, e: {}", pi, e);
    println!("Size of pi is {} bytes", std::mem::size_of_val(&pi));
    println!("Size of e is {} bytes", std::mem::size_of_val(&e));

    println!("Max f64: {}", std::f64::MAX);
    println!("Min f64: {}", std::f64::MIN);

    println!("Max f32: {}", std::f32::MAX);
    println!("Min f32: {}", std::f32::MIN);
}

fn boolean() {
    let is_active: bool = true;
    let is_admin: bool = false;

    println!("is_active: {}, is_admin: {}", is_active, is_admin);

    println!("Size of is_active is {} bytes", std::mem::size_of_val(&is_active));
    println!("Size of is_admin is {} bytes", std::mem::size_of_val(&is_admin));
}

fn char() {
    let a: char = 'a';
    let b: char = 'b';
    let c: char = 'c';

    println!("a: {}, b: {}, c: {}", a, b, c);
    println!("Size of a is {} bytes", std::mem::size_of_val(&a));
    println!("Size of b is {} bytes", std::mem::size_of_val(&b));
    println!("Size of c is {} bytes", std::mem::size_of_val(&c));
}

fn main() {
    // integer();
    // float();
    // boolean();
    // char();
}
