use rpds::HashTrieMap;

fn main() {
    let n = part1(&vec![0,1,5,10,3,12,19]);
    println!("Part1: {}", n);

    let n = part2(&vec![0,1,5,10,3,12,19]);
    println!("Part2: {}", n);
}


fn part1(input: &Vec<u64>) -> u64 {
	find_nth(input, 2020)
}

fn part2(input: &Vec<u64>) -> u64 {
	find_nth(input, 30_000_000)
}

fn find_nth(input: &Vec<u64>, n: usize) -> u64 {
	let mut working = HashTrieMap::new();
	let mut next_working = HashTrieMap::new();
	let mut prev = 0;

	for i in 0..n {
		if i % 10000 == 0{
			print!("Working on {}\r", i);
		}
		let value = match input.get(i) {
			Some(n) => *n as usize,
			None => {
				if let Some(p) = working.get(&prev) {
					i - p - 1
				}
				else {
					0
				}
			}
		};
		working = next_working;
		prev = value;
		//We last saw this value at this index
		next_working = working.insert(value, i);
	}
	
	prev as u64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&vec![0,3,6]), 436);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&vec![0,3,6]), 175594);
	}
}