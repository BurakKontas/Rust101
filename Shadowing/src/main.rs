fn main() {
    let x: i32 = 5;

    let x: i32 = x + 1; // we are not changing the value of x, we are creating a new variable x and shadowing the previous one
    // x = x + 1; // this will give an error because x is immutable
    
    {
        let x: i32 = x * 2; // we are creating a new variable x and shadowing the previous one but this time in a inner scope
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // this will print the value of x that was created in the first line because the x in the inner scope is a different variable
}
