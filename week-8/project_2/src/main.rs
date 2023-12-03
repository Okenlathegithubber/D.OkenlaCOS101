// Rust program to find the person with the highest years of programming experience.

use std::io;

fn main() 
{
    let mut workers_name : Vec<String> = Vec::new();
    let mut workers_exp : Vec<usize> = Vec::new();
    
    let mut input = String::new();
    println!("How many developers are applying for the job interview?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let workers_applied: usize = input.trim().parse().expect("Invalid input");

    for _workers in 0..workers_applied
    {
        let mut input = String::new();
        println!("What is your name?");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let name : String = input.trim().parse().expect("Invalid input");
        workers_name.push(name);

        let mut input = String::new();
        println!("How long have you been programming for? (in years)");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let programming_exp:usize = input.trim().parse().expect("Invalid input");
        workers_exp.push(programming_exp);
    }
    let max_index = max(&workers_applied, &workers_exp);

    println!("{} has the most programming experience with {} years.", workers_name[max_index], workers_exp[max_index]);
}
fn max(size : &usize, vect: &Vec<usize>) -> usize
{
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..*size 
    {
        if vect[i] > max
        {
            max = vect[i];
            max_index = i;
        }
    }
    return max_index;
}