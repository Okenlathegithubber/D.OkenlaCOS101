// For loop illustrations
// Rust program to count numbers

use std::io;

fn main() {

    // Creating a variable and Getting the input for lower and upper bound
    println!("Enter lower bound");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");

    println!("Enter upper bound");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");

    // Creating a for loop to count the numbers in between the lower and upper bound.
    // where the upper_bound is not inclusive in the counting.
    for x in lower_bound..upper_bound
    {
        println!("Count Level is {}",x);
    }
}
