#![feature(str_split_once)]
#![feature(int_bits_const)]

use std::io::{self, Read};
use std::collections::HashMap;

type Mask = (u64, u64); //(1s, floating)
type MemIns = (u64, u64); //(index, value)

enum Instruction {
	Mask(Mask),
	Mem(MemIns)
}

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

    Ok(())
}

fn part1(input: &str) -> u64 {
	let mut mask: Mask = (0, 0);
	let mut mem: HashMap<u64, u64> = HashMap::new();

	for line in input.lines() {

		match parse_line(line) {
			Instruction::Mask(m)  => mask = m,
			Instruction::Mem(m) => {
				for addr in apply_mask(m.0, mask) {
					mem.insert(addr, m.1);
				}
			},
		};
	}

	mem.values().sum()
}

fn parse_line(line: &str) -> Instruction {
	let (cmd, val) = line.split_once(" = ").unwrap();

	if cmd == "mask" {
		let mask = val.chars().rev().enumerate().fold((0, 0), |(ones, floating), (i, bit)| 
			match bit {
				'1' => (ones + (1<<i), floating),
				'X' => (ones, floating + (1<<i)),
				_ => (ones, floating)
			});
		return Instruction::Mask(mask);
	}

	// Mem instruction
	Instruction::Mem((
		u64::from_str_radix(&cmd.chars().filter(|c| c.is_numeric()).collect::<String>(),10).unwrap(),
		u64::from_str_radix(val, 10).unwrap()))
}

fn apply_mask(addr: u64, mask: Mask) -> Vec<u64> {
	apply_mask_rec(vec![addr | mask.0], mask.1)
}

fn apply_mask_rec(addrs: Vec<u64>, mask: u64) -> Vec<u64> {
	match (0..u64::BITS).filter(|i| (1<<i) & mask != 0).nth(0) {
		Some(i) => apply_mask_rec(addrs.iter().flat_map(|a|vec![a | (1<<i), a & (!(1<<i))]).collect(), mask & (!(1<<i))),
		None => addrs
	}
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_part1() {
        let input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

        assert_eq!(part1(input), 208);

    }

}
