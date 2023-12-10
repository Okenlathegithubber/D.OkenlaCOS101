// Rust program to merge seperate datasets into one single output.

use std::io;
use std::io::Write;

fn main() 
{
    let mut file = std::fs::File::create("COMMISIONER.txt").expect("create failed");

    file.write_all("S/N\t\t\t\tNAME OF COMMISIONER \t\t\t\t\tMINISTRY \t\t\t\tGEOPOLITICAL ZONE\n".as_bytes()).expect("Failed to write");

    let mut num : Vec<String> = Vec::new();
    let mut commisioner_name : Vec<String> = Vec::new();
    let mut ministry : Vec<String> = Vec::new();
    let mut geo_political_zone : Vec<String> = Vec::new();

    println!("How many convicted ministers are been registered");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ministers : usize = input.trim().parse().expect("Invalid input");

    for count in 0..ministers
    {
        let s_n = count + 1;
        num.push(s_n.to_string());
        file.write_all(s_n.to_string().as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is the Minister's {} name?", count+1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        let string_name = name.trim();
        commisioner_name.push(string_name.to_string());
        file.write_all(string_name.as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is the Minister's {} Ministry?", count+1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _ministry = input.trim();
        ministry.push(_ministry.to_string());
        file.write_all(_ministry.as_bytes()).expect("Failed to write");
        file.write_all("\t\t\t".as_bytes()).expect("Failed to write");

        println!("\nWhat is the Minister's {} Geo-Political Zone?", count+1);
        let mut zone = String::new();
        io::stdin().read_line(&mut zone).expect("Failed to read input");
        let _zone = zone.trim();
        geo_political_zone.push(_zone.to_string());
        file.write_all(_zone.as_bytes()).expect("Failed to write");
        file.write_all("\n".as_bytes()).expect("Failed to write");
    }

    println!("\nData Written!!");
}
