// Rust program to read the height of a person
// and then print if person is tall, a dwarf
// or average height.

use std::io;

fn main() 
{
    // Creating a variable for height.
    let mut input = String::new();

    // Getting the input for Height.
    println!("Enter your Height (in cm): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    // Checking if the height is up to limit to be called tall, a dwarf or average.
    // If yes,print command.
    // else height is lower than a range.
    // Print command.
    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf");
    }
    else
    {
        println!("Abnormal Height");
    }
}
