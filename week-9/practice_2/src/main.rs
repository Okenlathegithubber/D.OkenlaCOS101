// Reading a file

use std::io::Read;
use std::io::Write;

fn main() 
{
    let mut _file = std::fs::File::create("welcome_message.txt").expect("create failed");

    let mut file = std::fs::File::open("welcome_message.txt").unwrap(); // Without the file that was created above there would be an error about the file not being created.

    println!("Type any contents?");
    let mut contents = String::new();
    std::io::stdin().read_line(&mut contents).expect("Failed to read input");

    _file.write_all(contents.as_bytes()).expect("write failed");

    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    // Note: To comment the code line 7 or any line of code, "Ctrl" + "/".
}
