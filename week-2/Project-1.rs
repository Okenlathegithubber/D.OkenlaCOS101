// Compound Interest
fn main() {
	// values assinged to each variables
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	//Compund Interest
	let a = p * (1.0 + (r/100.0)) * n;

	println!("Amount is N{}", a);
	
	let ci = a - p;
	println!("Compound Interest is N{}", ci);
}