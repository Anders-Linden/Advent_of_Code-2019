fn part1() -> &'static str {
	return "Skeleton for part 1";
}

fn part2() -> &'static str {
	return "Skeleton for part 2";
}

fn main() {
	println!("Results for template");
	println!("-------------------");
	println!("Part 1: {}", part1());
	println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main(), ());
	}
}
