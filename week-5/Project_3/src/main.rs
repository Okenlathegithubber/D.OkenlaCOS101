use std::io;

fn main() 
{
    println!("WELCOME TO TABITHA KITCHEN!!!");
    println!("");

    let p = "Poundo Yam/Edinkaiko Soup";
    let f = "Fried Rice and Chicken";
    let a = "Amala and Ewedu Soup";
    let e = "Eba and Egusi Soup";
    let w = "White Rice and Stew";
    let s = "stop";
    let mut choice = String::new();
    let mut portion = String::new();

    let price_p = 3200; 
    let price_f = 3000;
    let price_a = 2500;
    let price_e = 2000;
    let price_w = 2500;

    println!("Menu:\n{} N{}\n{} N{}\n{} N{}\n{} N{}\n{} N{}",p,price_p,f,price_f,a,price_a,e,price_e,w,price_w);
    println!("");

    println!("Enter p for {}\nEnter f for {}\nEnter a for {}\nEnter e for {}\nEnter w for {}\nEnter s for {}",p,f,a,e,w,s);
    println!("What would you like to have?");
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    println!("How many portions?");
    io::stdin().read_line(&mut portion).expect("Failed to read input");
    let portion:i32 = portion.trim().parse().expect("Failed to input");


    loop
    {
        println!("Would you like to have anything else? (yes/no)");
        io::stdin().read_line(&mut choice).expect("Failed to read input");
    }
    // loop
    // {
    //     println!("Would you like to have anything else? (yes/no)");
    //     io::stdin().read_line(&mut choice).expect("Failed to read input");
    //     if choice == "yes"
    //     {
    //         println!("What else would you like?");
    //         io::stdin().read_line(&mut choice).expect("Failed to read input");
    //         println!("How many portions?");
    //         io::stdin().read_line(&mut portion).expect("Failed to read input");
    //         let portion:i32 = portion.trim().parse().expect("Failed to input");
    //     }

    // }    
}
