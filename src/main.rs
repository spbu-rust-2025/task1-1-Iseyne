use std::io;


fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Reading error");

	let nums: Vec<i32> = input
		.trim()
		.split_whitespace()
		.map(|s| s.parse().expect("Error: user entered something other than a number"))
		.collect();

	print!("{}", nums[0] + nums[1]);
}
