// Deleting a file

use std::io::Write;

use std::fs;

fn main() 
{

    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Softwaare Engineering";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");

    fs::remove_file("data.txt").expect("could no remove file");
    // remove_file() is the function used to delete the file created.
    println!("file is removed");

    // Note: If the remove_file function is commented, line 19 - 21, the file called "data.txt" will be created.
}
