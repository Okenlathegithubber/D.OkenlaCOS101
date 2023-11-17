//Rust Program to make a multiplication table
use std::io;

fn main() 
{
    println!("What should be the max number?");
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Failed to read input");
    let _max:i32 = max.trim().parse().expect("Failed to input");
    println!("");

    println!("What should be the multiplication number?");
    let mut mult_num = String::new();
    io::stdin().read_line(&mut mult_num).expect("Failed to read input");
    let _mult_num:i32 = mult_num.trim().parse().expect("Failed to input");
    println!("");
    for n in 1.._max + 1
    {
        let answer = n * _mult_num;
        println!("{} X {} = {}",_mult_num, n, answer);
    }
}
