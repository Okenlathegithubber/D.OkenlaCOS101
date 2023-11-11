// Rust program to determine age pass

use std::io;

fn main() 
{
    // Creating a variable.
    let mut input1 = String::new();
    let mut input2 = String::new();
    let age_limit = 18.0;

    // Getting the input for name and age.
    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    
    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    // Checking if the age is > 18.
    // If yes,print command.
    if age >= age_limit
    {
        println!("Welcome to the party {}!!!", input1);
    }
    else 
    {
        println!("Oops, you are not of age to enter the party {}", input1);
    }
}
