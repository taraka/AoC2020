#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::HashMap;

type Mask = (u64, u64); //(1s, 0s)
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
				println!("Mask: ({:b}, {:b})\nmem[{}] = {}\n{}\n\n", mask.0, mask.1, m.0, m.1, (m.1 | mask.0) & mask.1);
				mem.insert(m.0, m.1 | mask.0 & mask.1);
			},
		};

	}


	println!("{:#?}", mem);
	mem.values().sum()
}

fn parse_line(line: &str) -> Instruction {
	let (cmd, val) = line.split_once(" = ").unwrap();

	if cmd == "mask" {
		let mask = val.chars().rev().enumerate().fold((0, u64::MAX), |(ones, zeros), (i, bit)| 
			match bit {
				'1' => (ones + (1<<i) , zeros),
				'0' => (ones, zeros - (1<<i)),
				_ => (ones, zeros)
			});
		return Instruction::Mask(mask);
	}

	// Mem instruction
	Instruction::Mem((
		u64::from_str_radix(&cmd.chars().filter(|c| c.is_numeric()).collect::<String>(),10).unwrap(),
		u64::from_str_radix(val, 10).unwrap()))
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_part1() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

        assert_eq!(part1(input), 165);

    }

}

//Not: 6638226208555
//Not: 6638226208555
