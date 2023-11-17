use std::io;

fn main() 
{
    println!("WELCOME TO TABITHA KITCHEN!!!");
    println!("");
    println!("Note: Discount of 5% is given when price is more than N10,000");
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup      - N3,200");
    println!("F = Fried Rice & Chicken           - N3,000");
    println!("A = Amala & Ewedu Soup             - N2,500");
    println!("E = Eba & Egusi Soup               - N2,000");
    println!("W = White Rice & Stew              - N2,500");

    let P = "Poundo Yam/Edinkaiko Soup";
    let F = "Fried Rice & Chicken";
    let A = "Amala & Ewedu Soup";
    let E = "Eba & Egusi Soup";
    let W = "White Rice & Stew";

    let price_P = 3200.0;
    let price_F = 3000.0;
    let price_A = 2500.0;
    let price_E = 2000.0;
    let price_W = 2500.0;

    println!("What would you like to order?");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim();

    println!("How many portion(s) of {} would you like?",food_type);
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: f32 = quantity.trim().parse().expect("Failed to input");

    loop
    {
        println!("Would you like to order anything else? (yes/no)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        if choice == "yes"
        {

            println!("What would you like to order?");
            let mut food_type = String::new();
            io::stdin().read_line(&mut food_type).expect("Failed to read input");
            let food_type = food_type.trim();

            println!("How many portion(s) of {} would you like?",food_type);
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read input");
            let quantity: f32 = quantity.trim().parse().expect("Failed to input");

            if food_type == "P"
            {
                let total_P = quantity * price_P;
                println!("Total charge for {}: {}", P, total_P);
                if total_P > 10_000.0
                {
                    let discount_price = total_P * 0.05;
                    println!("The discount is {}", discount_price);
                    let discount = total_P - discount_price;
                    println!("Your Total price now:{}", discount);
                }
            }

            else if food_type == "F"
            {
                let total_F = quantity * price_F;
                println!("Total charge for {}: {}", F, total_F);
                if total_F > 10_000.0
                {
                    let discount_price = total_F * 0.05;
                    println!("The discount is {}", discount_price);
                    let discount = total_F - discount_price;
                    println!("Your Total price now: {}", discount);
                }
            }

            else if food_type == "A"
            {
                let total_A = quantity * price_A;
                println!("Total charge for {}: {}", A, total_A);
                if total_A > 10_000.0
                {
                    let discount_price = total_A * 0.05;
                    println!("The discount is {}", discount_price);
                    let discount = total_A - discount_price;
                    println!("Your Total price now: {}", discount);
                }
            }

          else if food_type == "E"
            {
                let total_E = quantity * price_E;
                println!("Total charge for {}: {}", E, total_E);
                if total_E > 10_000.0
                {
                    let discount_price = total_E * 0.05;
                    println!("The discount is {}", discount_price);
                    let discount = total_E - discount_price;
                    println!("Your Total price now: {}", discount);
                }
            }

            else if food_type == "W"
            {
                let total_W = quantity * price_W;
                println!("Total charge for {}: {}", W, total_W);
                if total_W > 10_000.0
                {
                    let discount_price = total_W * 0.05;
                    println!("The discount is {}", discount_price);
                    let discount = total_W - discount_price;
                    println!("Your Total price now: {}", discount);
                }
            }
            else if food_type.len() > 1
            {
                println!("HELLO");
                let total_P = quantity * price_P;
                let total_F = quantity * price_F;
                let total_A = quantity * price_A;
                let total_E = quantity * price_E;
                let total_W = quantity * price_W;


            } 
        }
        else 
        {
            break;
        }    
    }
}