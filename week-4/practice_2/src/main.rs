fn main() {
    let empty_string = String::new();   //"String::new()" is a String Object used to create a new string.
    println!("Length of String is {}",empty_string.len());

    let content_string = String::from("Computer Science");
    println!("Length of String is {}",content_string.len());
    //Note: The Difference between "String::new" and "String::from" is that in String::new you can't write a string in the bracket.
}
