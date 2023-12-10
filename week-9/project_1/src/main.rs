// Rust program to create a file thats saves information in a tabular format.

use std::io::Write;

fn main() 
{
    let drinks = "LAGER           STOUT           NON-ALCOHOLIC\n
33 Export       Legend          Maltina
Desperados      Turbo King      Amstel Malta
Goldberg        Williams        Malta Gold
Gulder                          Fayrouz
Heineken                
Star";
    
    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all(drinks.as_bytes()).expect("write failed");

    println!("Data written!");
}
