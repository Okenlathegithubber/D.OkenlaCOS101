// Passing a struct to a function

// declare a structure
struct Employee
{
    ceo:String,
    company:String,
    age:u32
}

fn main() 
{
    // Initialize a structure
    let emp1 = Employee
    {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee
    {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    // pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

// fetch values of speecific structure fields using the
// operator and print it to the console
fn display(emp:Employee)
{
    println!("Name: {}, Company: {}, Age: {}", emp.ceo, emp.company, emp.age);
}
