#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_labels)]
#![allow(unreachable_code)]

fn main() {
    for_loop();
    // while_loop();
    // loop_loop();
    // loop_labels();
    // loop_labels_with_return();
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_loop() {
    // loop {
    //     println!("Loop forever!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // 20
    println!("The counter is {}", counter); // 10
}

fn loop_labels() {'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            // break;

            // This would break the outer loop
            break 'outer; 
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn loop_labels_with_return() {
    let a = 'outer: loop {
        println!("Entered the outer loop");

        let b = 'inner: loop {
            println!("Entered the inner loop");

            // This would break the outer loop
            break 'outer 10;
        };

        println!("This point will never be reached");
    };

    println!("Exited the outer loop with value: {}", a);
}