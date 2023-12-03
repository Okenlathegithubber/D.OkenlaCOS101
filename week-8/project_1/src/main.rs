// Rust program to validate Staff level using vectors.
fn main() 
{
    // Creating an empty vector string
    let _public_servant = vec!["APS 1-2","APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let _job = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];

    let _office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];

    let _academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecutrer", "Dean"];

    let _lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    let _teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Welcome to the Public Service APS level checker !!!");
    println!("");
    println!("The Public Service APS level checker only checks the APS for {}, {}, {}, {}", _job[0], _job[1], _job[2], _job[3]);
    println!("");
    println!("How many staffs are checking their staff level?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let _staff_num:i32 = input.trim().parse().expect("Invalid input");

    for _staffs in 0.._staff_num
    {
        println!("\nEnter 1 for {}", _job[0]);
        println!("Enter 2 for {}", _job[1]);
        println!("Enter 3 for {}", _job[2]);
        println!("Enter 4 for {}", _job[3]);
        println!("");
        println!("What is your career choice ?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let _staff_career:i32 = input.trim().parse().expect("Invalid Input");

        println!("What is your work experience ?");
        let mut input_1 = String::new();
        std::io::stdin().read_line(&mut input_1).expect("Failed to read input");
        let _staff_experience:i32 = input_1.trim().parse().expect("Invalid input");

        if _staff_career == 1 || _staff_career == 2 || _staff_career == 3 || _staff_career == 4
        {
            println!("What is your occupation ?");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let staff_career_level = input.trim();

            if staff_career_level == _office_admin[0] || staff_career_level == _lawyer[0] || staff_career_level == _teacher[0] && _staff_experience > 1 && _staff_experience <= 2
            {
                println!("You hold the position of {}", _public_servant[0]);
            }

            else if staff_career_level == _office_admin[1] || staff_career_level == _academic[1] || staff_career_level == _lawyer[1] || staff_career_level == _teacher[1] && _staff_experience > 3 && _staff_experience <= 5
            {
                println!("You hold the position of {}", _public_servant[1]);
            }

            else if staff_career_level == _office_admin[2] || staff_career_level == _academic[2] || staff_career_level == _lawyer[2] || staff_career_level == _teacher[2] && _staff_experience > 5 && _staff_experience <= 8
            {
                println!("You hold the position of {}", _public_servant[2]);
            }

            else if staff_career_level == _office_admin[3] || staff_career_level == _academic[3] || staff_career_level == _lawyer[3] || staff_career_level == _teacher[3] && _staff_experience > 8 && _staff_experience <= 10
            {
                println!("You hold the position of {}", _public_servant[3]);
            }

            else if staff_career_level == _office_admin[4] || staff_career_level == _academic[4] || staff_career_level == _lawyer[4] || staff_career_level == _teacher[4] && _staff_experience > 10 && _staff_experience <= 13
            {
                println!("You hold the position of {}", _public_servant[4]);
            }

            else if staff_career_level == _office_admin[5] || staff_career_level == _academic[5] || staff_career_level == _lawyer[5] || staff_career_level == _teacher[5] && _staff_experience > 13
            {
                println!("You hold the position of {}", _public_servant[5]);
            }

            else 
            {
                println!("Sorry.. Unfortunately You do not hold any public service postition.");
            }
        }
    }
}
