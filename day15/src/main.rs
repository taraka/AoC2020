use std::collections::HashMap;

fn main() {
    let n = part1(&[0,1,5,10,3,12,19]);
    println!("Part1: {}", n);

    let n = part2(&[0,1,5,10,3,12,19]);
    println!("Part2: {}", n);
}


fn part1(input: &[u64]) -> u64 {
	find_nth(input, 2020)
}

fn part2(input: &[u64]) -> u64 {
	find_nth(input, 30_000_000)
}

fn find_nth(input: &[u64], n: usize) -> u64 {
	let mut working: HashMap<usize, Vec<usize>> = HashMap::new();
	let mut prev = 0;

	for i in 0..n {
		let list = { 
			if let Some(l)  = working.get_mut(&prev) {
				l
			} else {
				let new = Vec::new();
				working.insert(prev, new);
				working.get_mut(&prev).unwrap()
			}
		};


		let value = if i < input.len() {
			input[i] as usize
		}
		else {
			if list.len() as i64 - 1 < 0 {
				0
			}
			else if let Some(p) = list.get(list.len()-1) {
				i - p
			}
			else {
				0
			}
		};
		//println!("Value: {}", value);
		prev = value;
		list.push(i);
	}
	
	prev as u64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&[0,3,6]), 436);
	}

	#[test]
	fn test_part2() {
		//assert_eq!(part2(&[0,3,6]), 175594);
	}
}