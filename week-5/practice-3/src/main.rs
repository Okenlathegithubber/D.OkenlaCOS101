// Rust Program to Calculate the area of a

use std::io;

fn main() 
{
    // Creating a variable.
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Getting the input for the base and height.
    println!("Enter base: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter height: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let height:f32 = input2.trim().parse().expect("Not a valid number");

    // Checking if the base and height is > 0.
    // If yes, get the area.
    if base > 0.0 && height > 0.0
    {
        let area:f32 =(base * height) / 2.0;
        println!("Area of a triangle: {}", area);
    }
}
