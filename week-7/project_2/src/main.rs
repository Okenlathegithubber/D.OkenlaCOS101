 // Rust program to get and display data of sibling(s).

use std::io;

fn main() 
{
    let mut names : Vec<String> = Vec::new();
    let mut ages : Vec<i32> = Vec::new();
    let mut martial_status : Vec<String> = Vec::new();
    let mut university : Vec<String> = Vec::new();
    let mut occupation : Vec<String> = Vec::new();
    let mut course : Vec<String> = Vec::new();
    let mut offspring : Vec<String> = Vec::new();
    let mut location : Vec<String> = Vec::new();

    println!("Hey, What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let _name = name.trim();

    println!("Do you have any siblings? (Y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _sibling = input.trim();

    if _sibling == "Y"
    {
        println!("How many siblings do you have?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let sibling_amount:usize = input.trim().parse().expect("Invalid input");

        for _count in 0..sibling_amount
        {
            println!("What is his/her first name?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let _sibling_name : String = input.trim().parse().expect("Invalid input");
            names.push(_sibling_name);

            println!("How old is he/her?");
            let mut input_1 = String::new();
            io::stdin().read_line(&mut input_1).expect("Failed to read input");
            let _sibling_age : i32 = input_1.trim().parse().expect("Invalid input");
            ages.push(_sibling_age);


            if _sibling_age >= 18
            {
                println!("Is he/she single or married? (S/M)");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let _choice_martial_status : String = input.trim().parse().expect("Invalid input");
                martial_status.push(_choice_martial_status.clone());

                if _choice_martial_status == "S"
                {
                    println!("Is he/she a student or a worker? (S/W)");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    let _choice_occupation : String = input.trim().parse().expect("Invalid input");
                    occupation.push(_choice_occupation.clone());

                    if _choice_occupation == "S"
                    {
                        println!("What is he/her University?");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input");
                        let _sibling_college : String = input.trim().parse().expect("Invalid input");
                        university.push(_sibling_college);

                        println!("What is he/her Course of study?");
                        let mut input_1 = String::new();
                        io::stdin().read_line(&mut input_1).expect("Failed to read input");
                        let _sibling_course : String = input_1.trim().parse().expect("Invalid input");
                        course.push(_sibling_course);
                    }
                }
                else if _choice_martial_status == "M"
                {
                    println!("Does he/her have any offspring? (Y/N)");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    let _choice : String = input.trim().parse().expect("Invalid input");
                    offspring.push(_choice);

                    println!("Which city does he/her family live?");
                    let mut input_1 = String::new();
                    io::stdin().read_line(&mut input_1).expect("Failed to read input");
                    let _sibling_city : String = input_1.trim().parse().expect("Invalid input");
                    location.push(_sibling_city);
                }
            }
            else if _sibling_age < 18
            {
                println!("Have he/she written WAEC? (Y/N)");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let _waec : String = input.trim().parse().expect("Invalid input");

                if _waec == "Y"
                {
                    println!("Which Secondary School did he/she attend?");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    let _sibling_school : String = input.trim().parse().expect("Invalid input");
                }
                else
                {
                    println!("What is his/her current class level?");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                    let _sibling_class : String = input.trim().parse().expect("Invalid input");
                }
            }
        }
        for index in 0..sibling_amount
        {
            println!("\nName {}: {}", index + 1, names[index]);
        }
        for index in 0..sibling_amount
        {
            println!("Age {}: {}", index + 1, ages[index]);
        }
        for index in 0..sibling_amount
        {
            println!("Martial Status {}: {}", index + 1, martial_status[index]);
        
            if false
            {
                println!("Martial Status {}: -", index + 1);
            }
        }
        // for index in 0..sibling_amount
        // {

        //     println!("Occupation {}: {}", index + 1, occupation[index]);
        // }
        // for index in 0..sibling_amount
        // {
        //     println!("University {}: {}", index + 1, university[index]);
        // }
        // for index in 0..sibling_amount
        // {
        //     println!("Course {}: {}", index + 1, course[index]);
        // }
        // for index in 0..sibling_amount
        // {
        //    println!("Offspring {}: {}", index + 1, offspring[index]);
        // }

        // for index in 0..sibling_amount
        // {
        //     println!("Location {}: {}", index + 1, location[index]);
        // }
    }
}
