// Rust program to develop a Rust application that will help 
// manage the access to structures of the database schemas of the company.
use std::io;
use std::io::Read;

fn open_globacom_dbase()
{
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_project_tb()
{
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_staff_tb()
{
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_customer_tb()
{
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn open_data_plan_tb()
{
    let mut file = std::fs::File::open("data-plan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main()
{
    println!("Which is your position below:\n");
    println!("Enter 1 for Administrator");
    println!("Enter 2 for Project Manager");
    println!("Enter 3 for Employee");
    println!("Enter 4 for Customer");
    println!("Enter 5 for Vendor\n");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let position: i32 = choice.trim().parse().expect("Failed to input");

    if position == 1
    {
        println!("This is the Database structure");
        open_globacom_dbase();
    }

    if position == 2
    {
        println!("This is the Project table");
        open_project_tb();
    }

    if position == 3
    {
        println!("This is the Staff table");
        open_staff_tb();
    }

    if position == 4
    {
        println!("This is the Customer table");
        open_customer_tb();
    }

    if position == 5
    {
        println!("This is the Data-Plan table");
        open_data_plan_tb();
    }

}
