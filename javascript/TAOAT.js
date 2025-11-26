export function is_six(num) {
	let not_factor = 0;
	for (let i = 2; i < num; i++) {
		if (num % i != 0) { not_factor++; }
	}
	return not_factor == 2;
} 
