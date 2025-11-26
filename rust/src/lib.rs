pub fn is_six(num: i32) -> bool {
	let mut not_factor: i32 = 0;
	for i in 2..num {
		if num % i == 0 { not_factor += 1 }
	}
	not_factor == 2
}
