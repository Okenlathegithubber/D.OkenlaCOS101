// Rust program to check eligibility to vote.
use std::io;

fn main() 
{
    for _students in 0..151
    {
        println!("What is your name?");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let _name = name.trim();
        println!("");

        println!("What is your school e-mail?");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read input");
        let _email = email.trim();
        println!("");

        println!("What is your department?");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Failed to read input");
        let _department = department.trim();
        println!("");

        println!("What is your State of origin?");
        let mut s_of_o = String::new();
        io::stdin().read_line(&mut s_of_o).expect("Failed to read input");
        let _s_of_o = s_of_o.trim();
        println!("");

        println!("Are you a class rep? (Y/N)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read input");
        let _answer = answer.trim();
        println!("");

        println!("Are you a 100 level student? (Y/N)");
        let mut choice= String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let _choice= choice.trim();
        println!("");

        println!("What is your C.G.P.A?");
        let mut cgpa = String::new();
        io::stdin().read_line(&mut cgpa).expect("Failed to read input");
        let _cgpa:f32 = cgpa.trim().parse().expect("Failed to input");
        println!("");

        if _answer == "Y" && _choice == "N" && _cgpa > 4.0
        {
            println!("Name: {}, Email: {}, Department: {}, State of origin: {} ",_name, _email, _department, _s_of_o);
            println!("You can vote.");
        }
        else 
        {
            println!("Sorry your not eligible to vote.");
        }
    }
}
