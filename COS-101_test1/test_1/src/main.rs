use std::io;
use std::fs::File;
use std::io::Write;

struct Company
{
    name: String,
    year: i32,
    shares: i32,
    liabilities: i32
}

impl Company
{
    fn new(name: &str, year: i32, shares: i32, liabilities: i32) -> Self 
    {
        Company 
        {
            name: name.to_string(),
            year,
            shares,
            liabilities
        }
    }

    fn percentage_leverage(&self)->f32
    {
        ((self.shares - self.liabilities) as f32/ self.shares as f32) * 100.0
    }
}

fn main() 
{
    let companies = vec![
        Company::new("Cadbury", 1965, 15_000_000, 5_500_000),
        Company::new("Champion", 1974, 25_000_000, 8_000_000),
        Company::new("Dangote Sugar", 1970, 18_000_000, 10_000_000),
        Company::new("Flour Mills", 1960, 32_000_000, 4_000_000),
        Company::new("Nestle", 1961, 8_000_000, 1_500_000),
        Company::new("Unilever", 1923, 37_000_000, 11_000_000),
        Company::new("Honeywell", 1906, 34_000_000, 9_000_000),
        Company::new("Nigerian Breweries", 1946, 30_000_000, 12_000_000)
    ];

    println!("Enter username");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to input");
    let username: String = input.trim().to_string();

    if username.len() < 3 || username.len() > 8 
    {
        println!("Invalid username length");
        return;
    }

    let password = get_password();

    let sym = "$#@";
    let caps = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if !password_is_valid(&password) || password.chars().any(|c| sym.contains(c)) || password.chars().any(|c| caps.contains(c))
    {
        println!("Invalid Password...");
        return;
    }

    let mut leverage_file = File::create("leverage_data.txt").expect("Failed to create file");

    for company in &companies 
    {
        let leverage_percentage = company.percentage_leverage();
        if username == &company.name[..4] 
        {

            leverage_file.write_all("Company\n".as_bytes()).expect("Failed to write");
            leverage_file.write_all(company.name.as_bytes()).expect("Failed to write");
            leverage_file.write_all("\n\n".as_bytes()).expect("Failed to write");

            leverage_file.write_all("Founded Date\n".as_bytes()).expect("Failed to write");
            leverage_file.write_all(company.year.to_string().as_bytes()).expect("Failed to write");
            leverage_file.write_all("\n\n".as_bytes()).expect("Failed to write");

            leverage_file.write_all("Assets\n".as_bytes()).expect("Failed to write");
            leverage_file.write_all(company.shares.to_string().as_bytes()).expect("Failed to write");
            leverage_file.write_all("\n\n".as_bytes()).expect("Failed to write");

            leverage_file.write_all("Liabilities\n".as_bytes()).expect("Failed to write");
            leverage_file.write_all(company.liabilities.to_string().as_bytes()).expect("Failed to write");
            leverage_file.write_all("\n\n".as_bytes()).expect("Failed to write");

            leverage_file.write_all("Leverage Percentage\n".as_bytes()).expect("Failed to write");
            leverage_file.write_all(leverage_percentage.to_string().as_bytes()).expect("Failed to write");
            leverage_file.write_all("\n\n".as_bytes()).expect("Failed to write");
            

            if company.shares > 20_000_000 
            {
                leverage_file.write_all(leverage_percentage.to_string().as_bytes()).expect("Failed to write");// (leverage_file, "{:.2}", leverage_percentage).expect("Failed to write to file");
            }

            if company.liabilities < 10_000_000 
            {
                let bonus = leverage_percentage * 0.05;
                println!("Bonus for {}: {:.2}%", company.name, bonus);
            }
        }
    }
       
}

fn get_password() -> String 
{
    println!("Enter Password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    password.trim().to_string()
}

fn password_is_valid(password: &str) -> bool
{
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let num = "0123456789";
    password.chars().any(|c| letters.contains(c)) || password.chars().any(|c| num.contains(c))
}