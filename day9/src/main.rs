use std::io::{self, Read};
use itertools::Itertools;

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

	let input: Vec<u64> = buffer.lines()
                                .into_iter()
                                .filter_map(|x| u64::from_str_radix(x, 10).ok())
                                .collect();

    let n = part1(&input, 25);
    println!("Part1: {}", n);

    let n = part2(&input, 25);
    println!("Part2: {}", n);

    Ok(())
}


fn part1(input: &Vec<u64>, pre_len: usize) -> u64 {
	input[(pre_len..input.len()).find(|i| !check(input[*i], &input[i-pre_len..*i])).unwrap()]
}

fn part2(input: &Vec<u64>, pre_len: usize) -> u64 {
	let nums = find_part2_nums(input, part1(input, pre_len));
	nums.iter().min().unwrap() + nums.iter().max().unwrap()
}

fn find_part2_nums(input: &Vec<u64>, bad: u64) -> Vec<u64> {
	for i in 0..input.len() {
		let mut consider: Vec<u64> = Vec::new();
		for v in &input[i..] {
			consider.push(*v);
			let sum: u64 = consider.iter().sum();
			if sum == bad {
				return consider
			}
			else if sum > bad {
				break;
			}
		}
	}

	panic!("Didn't find it");
}

fn check(value: u64, list: &[u64]) -> bool {
	list.iter()
	.combinations(2)
	.map(|c| c.iter().fold(0, |acc, j| acc + *j))
	.any(|s: u64| s == value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
	    let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        assert_eq!(part1(&input, 5), 127);
        assert_eq!(part2(&input, 5), 62);

    }

}