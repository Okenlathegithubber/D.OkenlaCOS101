fn main() {
	// let t:f64 = 450000.00;
	// let m:f64 = 1500000.00;
	// let h:f64 = 750000.00;
	// let d:f64 = 2850000.00;
	// let a:f64 = 250000.00;

	let qt:f64 = 2.0;
	let qm:f64 = 1.0;
	let qh:f64 = 3.0;
	let qd:f64 = 3.0;
	let qa:f64 = 1.0;

	let t:f64 = qt * 450000.00;
	let m:f64 = qm * 1500000.00;
	let h:f64 = qh * 750000.00;
	let d:f64 = qd * 2850000.00;
	let a:f64 = qa * 250000.00;

	// sum of value
	let sum = t + m + h + d + a;
	println!("Sum of record sales is N{}", sum);

	//sum of quantity
	let qsum = qt + qm + qh + qd + qa;

	//Average of record sales
	let average = sum / qsum;
	println!("Average of record sales is N{}", average);

}