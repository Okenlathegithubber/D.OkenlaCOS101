fn main() {
    let name = "Aisha Lawal";   // "name" has no "&str".
    let uni:&str = "Pan-Atlantic University";   //"uni" has "&str". &str = String Literal
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAdress: {}",uni,addr);

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school);
}
