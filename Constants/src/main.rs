// In constants you are not allowed to use mut keyword
// Constants should be declared as uppercase with underscore between words due to rust naming rules

fn main() {
    // const mut MAX_POINTS: u32 = 100_000; // Syntax error: const globals cannot be mutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    println!("The value of PI is: {}", PI);

    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
}

// You can declare a constant in any scope including global scope
const PI: f32 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;