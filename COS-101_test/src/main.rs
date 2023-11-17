use std::io;

fn main() 
{
    println!("Welcome to Otunene Family Health Center!!!");
    println!("");
    println!("HEALTH DIAGNOSIS |AMOUNT(N) |VILLAGE
            A =  ALZHEIMER     |1,200,000 |A = AKPABOM
            B =  ARRHYTHMIA    |550,000   |B = NGBAUJI
            C =  CKD           |1,500,000 |C = ATABRIKANG
            D =  DIABETES      |800,000   |D = OKOROBILOMC            
            E = ARTHRITIS      |450,000   |E = EMEREMEN");

    println!("What is your name ?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to input");
    let name = name.trim();

    println!("How old are you?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to input");
    let age:f32 = age.trim().parse().expect("Failed to input");

    println!("What is your email address?");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to input");

    println!("What are you diagnosed with? (A,B,C,D,E)");
    let mut disease = String::new();
    io::stdin().read_line(&mut disease).expect("Failed to input");
    let disease = disease.trim();

     println!("How many children do you have?");
    let mut child_amount = String::new();
    io::stdin().read_line(&mut child_amount).expect("Failed to input");
    let child_amount:f32 = child_amount.trim().parse().expect("Failed to input");

    println!("How many siblings do you have?");
    let mut sibling_amount = String::new();
    io::stdin().read_line(&mut sibling_amount).expect("Failed to input");
    let sibling_amount:f32 = sibling_amount.trim().parse().expect("Failed to input");

    println!("Where do you live? (A,B,C,D,E)");
    let mut location = String::new();
    io::stdin().read_line(&mut location).expect("Failed to input");
    let location = location.trim(); 

    if disease == "A" && age > 50.0 && child_amount > 4.0 && location == "A"
    {
        let price:f32 = 1_200_000.0;
        println!("Your price is {}",price);

        let discount:f32 = price * 0.2;
        println!("You have a discount, Discount is {}",discount);

        let discounted_price:f32 = price - discount;
        println!("Your Discounted Price is {}",discounted_price);
    }

    else if disease == "B" && age == 30.0 && sibling_amount > 4.0 && location == "B"
    {
        let price:f32 = 550_000.0;
        println!("Your price is {}",price);

        let discount:f32 = price * 0.05;
        println!("You have a discount, Discount is {}",discount);

        let discounted_price:f32 = price - discount;
        println!("Your Discounted Price is {}",discounted_price);
    }

    else if disease == "C" && age > 40.0 && child_amount > 3.0  && sibling_amount > 3.0 && location == "C"
    {
        let price:f32 = 1_500_000.0;
        println!("Your price is {}",price);

        let discount:f32 = price * 0.15;
        println!("You have a discount, Discount is {}",discount);

        let discounted_price:f32 = price - discount;
        println!("Your Discounted Price is {}",discounted_price);

    }

    else if disease == "D" && age > 28.0 && age < 45.0 && child_amount > 2.0 && child_amount < 5.0 && location == "D"
    {
        let price:f32 = 800_000.0;
        println!("Your price is {}",price);

        let discount:f32 = price * 0.1;
        println!("You have a discount, Discount is {}",discount);

        let discounted_price:f32 = price - discount;
        println!("Your Discounted Price is {}",discounted_price);
    }
    // else {
    //     let price:f32 = 800_000.0;
    //     println!("Your price is {}",price);
    // }

    else if disease == "E" && age > 58.0 && child_amount > 5.0  && sibling_amount > 5.0 && location == "E"
    {
        let price:f32 = 450_000.0;
        println!("Your price is {}",price);

        let discount:f32 = price * 0.1;
        println!("You have a discount, Discount is {}",discount);

        let discounted_price:f32 = price - discount;
        println!("Your Discounted Price is {}",discounted_price);
    }

}
