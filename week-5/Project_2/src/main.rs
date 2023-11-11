use std::io;

fn main() 
{
   // Create variables for age and experience
   let mut age = String::new();
   let mut experience = String::new();
   // Getting user input for age and experience
   println!("How old are you?");
   io::stdin().read_line(&mut age).expect("Failed to read input");
   let age:i32 = age.trim().parse().expect("Failed to input");

   println!("Do you have work experience (yes/no)");
   io::stdin().read_line(&mut experience).expect("Failed to read input");
   let experience = experience.trim();


   // Compare if:
   // User is experienced and age >= 40
   // if yes print command.
   if experience == "yes" && age >= 40
   {
        println!("Your incentive is N1,560,000");
   }
   // User is experienced and 30 <= age < 40
   // if yes print command.
   else if experience == "yes" && age >= 30 && age < 40
   {
        println!("Your incentive is N1,480,000");
   }
   // User is experienced and age < 30
   // if yes print command.
   else if experience == "yes" && age < 30
   {
        println!("Your incentive is N1,300,000");
   }
   // Else print command.
   else 
   {
        println!("Your incentive is N100,000");
   }


}
