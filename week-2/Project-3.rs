fn main() {
	let p:u64 = 210000;
	let r:u64 = 5;
	let n:u64 = 3;

	// Getting amount.
	let a = p * (1 - (r/100)) ^ n;

	println!("Amount is N{}", a);

	//Getting the depreciation of the tv
	let ci = a - p;
	println!("Depriciation is N{}", ci);
}
