// Loop Illustration

fn main() {
    

    // while true

    // Creating the variable x
    let mut x = 0;

    // This loop is a function used to create a loop.
    loop 
    {
        // The term "x+=1", also means x = x + 1.
        // This means that x always increases by one until 
        // x == 15.
        x+=1;
        println!("x={}",x);

        if x==15 
        {
            break; //Break statement is used to stop the loop after limit
                   //of conditions has reached.
        }
    }
}
