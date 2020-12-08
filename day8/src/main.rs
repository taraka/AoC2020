#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::HashSet;
use rpds::Vector;

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

	let n = part2(&buffer);
    println!("Part2: {}", n);

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Op {
	ACC(i64),
	JMP(i64),
	NOP(i64)
}

#[derive(Debug)]
struct ProgramResult {
	acc: i64,
	success: bool
}

fn parse_instruction(line: &str) -> Option<Op> {
	let (ins_str, val_str) = line.split_once(' ')?;
	match ins_str {
		"acc" => Some(Op::ACC(i64::from_str_radix(val_str, 10).ok()?)),
		"jmp" => Some(Op::JMP(i64::from_str_radix(val_str, 10).ok()?)),
		"nop" => Some(Op::NOP(i64::from_str_radix(val_str, 10).ok()?)),
		_ => None
	}
}

fn parse_input(input: &str) -> Vector<Op> {
	input.lines()
		.filter_map(parse_instruction).collect()
}

fn part1(input: &str) -> i64 {
	let program = parse_input(input);
	run_program(&program).acc
}

fn part2(input: &str) -> i64 {
	let program = parse_input(input);
	run_program(&find_working(&program)).acc
}

fn run_program(program: &Vector<Op>) -> ProgramResult {
	let mut visited = HashSet::new();
	let mut acc: i64 = 0;
	let mut pc:i64 = 0;

	loop {
		if !visited.insert(pc) {
			return ProgramResult {
				acc: acc,
				success: false
			};
		}
		
		if let Some(op) = program.get(pc as usize) {
			match *op {
				Op::ACC(v) => { 
					acc += v;
					pc += 1;
				},
				Op::JMP(v) => pc += v,
				Op::NOP(_) => pc += 1
			}
		}
		else {
			return ProgramResult {
				acc: acc,
				success: true
			};
		}
	}
}

fn find_working(program: &Vector<Op>) -> Vector<Op> {
	for (i, o) in program.iter().enumerate() {
		let test = match o {
			Op::NOP(v) => program.set(i, Op::JMP(*v)),
			Op::JMP(v) => program.set(i, Op::NOP(*v)),
			_ => continue
		}.unwrap();

		if is_working(&test) {
			return test;
		}
	}

	panic!("Couldn't find a working answer");
}

fn is_working(program: &Vector<Op>) -> bool {
	run_program(program).success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;


        assert_eq!(part1(input), 5);

    }

    #[test]
    fn test_part2() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;


        assert_eq!(part2(input), 8);

    }
}