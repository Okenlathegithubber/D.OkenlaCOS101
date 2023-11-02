fn main() {
    let conv_d = 1.609;
    let d1:f32 = 80.0 * conv_d;
    let t1:f32 = 2.0;
    let d2:f32 = 120.0 * conv_d;
    let t2:f32 = 4.0;
    let speed1:f32 = d1/t1;
    let speed2:f32 = d2/t2;

    println!("The speed for formula 1 is {} km/hr",speed1);
    println!("The speed for Needs for speed is {} km/hr",speed2);
}
