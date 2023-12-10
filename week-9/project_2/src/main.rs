// Rust program that reads personal details of a student and save details into a file.

use std::io;
use std::io::Write;


fn main() 
{
    let mut file = std::fs::File::create("PAU_SMIS.txt").expect("create failed");

    file.write_all("\t\t\t\t\t\t\t\tPAU SMIS\n".as_bytes()).expect("Failed to write");

    file.write_all("Student Name\t\t\tMatric. Number\t\tDepartment\t\t\tLevel\n".as_bytes()).expect("Failed to write");

    let mut student_name : Vec<String> = Vec::new();
    let mut student_matric : Vec<String> = Vec::new();
    let mut student_dept : Vec<String> = Vec::new();
    let mut student_lvl : Vec<String> = Vec::new();

    println!("How many student are been checked?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let students : usize = input.trim().parse().expect("Invalid input");

   for amount in 0..students
   {
        println!("\nWhat is your name student {}?", amount+1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let string_name = name.trim();
        student_name.push(string_name.to_string());
        file.write_all(string_name.as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is your matriculation number student {}?", amount+1);
        let mut matric = String::new();
        io::stdin().read_line(&mut matric).expect("Failed to read input");
        let int_matric = matric.trim();
        student_matric.push(int_matric.to_string());
        file.write_all(int_matric.as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is your Department student {}?", amount+1);
        let mut dept = String::new();
        io::stdin().read_line(&mut dept).expect("Failed to read input");
        let string_dept = dept.trim();
        student_dept.push(string_dept.to_string());
        file.write_all(string_dept.as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is your level student {}?", amount+1);
        let mut lvl = String::new();
        io::stdin().read_line(&mut lvl).expect("Failed to read input");
        let int_lvl = lvl.trim();
        student_lvl.push(int_lvl.to_string());
        file.write_all(int_lvl.as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to write");
   }

   println!("\nData Written!!");
}
