use std::io;

fn main(){
	println!("rust trickery! this one can find common denominators of 2 positive integers. go ahead, type em out!");
	let str_thingy: String = String::from("hey mate wth");git
	loop {
		let mut input1 = String::new();
		let mut input2 = String::new();
		println!("Enter 2 numbers!");
		io::stdin()
			.read_line(&mut input1)
			.expect("ey man wtf");
		io::stdin()
			.read_line(&mut input2)
			.expect("ey man wtf");
		let input1: u32 = match input1.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("bro enter a positive integer dammit (returning value: 0)");
				0
			},
		};
		let input2: u32 = match input2.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("bro enter a positive integer dammit (returning value: 0)");
				0
			},
		};
		find_common_denominators_of_2_nums(input1, input2);
	}
}
fn find_common_denominators_of_2_nums(x: u32, y: u32) {
	let mut common_denominator_count: u32 = 0;
	let bigger_number: u32 = if x > y {x} else {y};
	let amount_of_iterations: u32= if bigger_number % 2 == 0{
		bigger_number/2
	}
	else {
		((bigger_number) / 2) as u32
	};
	println!("List of all common denominators (except for 1) of {} and {}: ",x,y);
	if x != y {
		for num in 2..amount_of_iterations {
			if (x % num == 0) && (y % num == 0) {
				common_denominator_count +=1;
				println!("{common_denominator_count}. {num}");
			}
		}
	} else {
		println!("They're the same lol")
	}

	println!("Amount of common denominators: {}", common_denominator_count)
}