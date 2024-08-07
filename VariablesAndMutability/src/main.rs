#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

fn main() {
    let a: u16 = 5;
    println!("a = {}", a);
    // a = 10; // error: cannot mutate immutable variable `a`

    let mut b: u16 = 10;
    b = 15;
    println!("b = {}", b);
}
