use std::io;
use std::io::Write;

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
	find_nth(input, 30000000)
}

fn find_nth(input: &Vec<u64>, n: usize) -> u64 {
	let mut working = Vec::with_capacity(n);

	for i in 0..n {
		if i % 10000 == 0{
			print!("Working on {}\r", i);
			io::stdout().flush();

		}
		working.insert(i, match input.get(i) {
			Some(n) => *n as usize,
			None => {
				if let Some((p, _v))  = working[..i-1].iter().enumerate().rev().find(|(_p, v)| *v == &working[i-1]) {
					i - p - 1
				}
				else {
					0
				}
			}
		});
	}
	println!("\n");
	*working.last().unwrap() as u64
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