fn main() {

    let fullname = "David Abiola Okenla";
    let department = "Software Engineering";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is:{}",fullname);
    //Check Length
    println!("The Length of my fullname is: {}",fullname.len());
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",uni);
    //Note: you can only push a string when the variable is mutable.
}
