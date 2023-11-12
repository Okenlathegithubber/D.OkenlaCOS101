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

    if food_type == "P"
    {
        let total = quantity * price_P;
        println!("Total charge for {}: {}", P, total);
        if total > 10_000.0
        {
            let discount_price = total * 0.05;
            println!("The discount is {}", discount_price);
            let discount = total - discount_price;
            println!("Your Total price now:{}", discount);
        }
    }

    else if food_type == "F"
    {
        let total = quantity * price_F;
        println!("Total charge for {}: {}", F, total);
        if total > 10_000.0
        {
            let discount_price = total * 0.05;
            println!("The discount is {}", discount_price);
            let discount = total - discount_price;
            println!("Your Total price now: {}", discount);
        }
    }

    else if food_type == "A"
    {
        let total = quantity * price_A;
        println!("Total charge for {}: {}", A, total);
        if total > 10_000.0
        {
            let discount_price = total * 0.05;
            println!("The discount is {}", discount_price);
            let discount = total - discount_price;
            println!("Your Total price now: {}", discount);
        }
    }

    else if food_type == "E"
    {
        let total = quantity * price_E;
        println!("Total charge for {}: {}", E, total);
        if total > 10_000.0
        {
            let discount_price = total * 0.05;
            println!("The discount is {}", discount_price);
            let discount = total - discount_price;
            println!("Your Total price now: {}", discount);
        }
    }

    else if food_type == "W"
    {
        let total = quantity * price_W;
        println!("Total charge for {}: {}", W, total);
        if total > 10_000.0
        {
            let discount_price = total * 0.05;
            println!("The discount is {}", discount_price);
            let discount = total - discount_price;
            println!("Your Total price now: {}", discount);
        }
    }

}