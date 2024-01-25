// Rust program to to calculate the total cost of items bought.

struct Eshop
{
    laptop1:String,
    laptop2:String,
    laptop3:String,
    laptop4:String,
    price1:u32,
    price2:u32,
    price3:u32,
    price4:u32
}

impl Eshop
{
    fn total(&self)->u32
    {
        self.price1 + self.price2 + self.price3 + self.price4
    }
}

fn main() 
{
    let e_shop = Eshop
    {
        laptop1:String::from("HP"),
        laptop2:String::from("IBM"),
        laptop3:String::from("Toshiba"),
        laptop4:String::from("Dell"),
        price1:650_000 * 3,
        price2:755_000 * 3,
        price3:550_000 * 3,
        price4:850_000 * 3

    };

    println!("Price of 3 {} Laptops is {}", e_shop.laptop1, e_shop.price1);
    println!("Price of 3 {} Laptops is {}", e_shop.laptop2, e_shop.price2);
    println!("Price of 3 {} Laptops is {}", e_shop.laptop3, e_shop.price3);
    println!("Price of 3 {} Laptops is {}", e_shop.laptop4, e_shop.price4);
    println!("Total for all is {}", e_shop.total());
}

