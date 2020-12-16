#![feature(str_split_once)]

use std::io::{self, Read};
use std::ops::RangeInclusive;

type Ticket = Vec<u64>;


#[derive(Debug, Clone)]
struct Rule<'a> {
	ranges: Vec<RangeInclusive<u64>>,
	field: &'a str
}

impl<'a> Rule<'a> {
	fn check_value(&self, value: &u64) -> bool {
		self.ranges.iter().any(|r| r.contains(value))
	}
} 

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

    let n = part2(&buffer);
    println!("Part2: {}", n);

    Ok(())
}

fn part1(input: &str) -> u64 {
	let (rules, _my_ticket, tickets) = parse_input(input);

	tickets.iter().flat_map(|t| find_invalid(t, &rules)).sum()
}

fn part2(input: &str) -> u64 {
	let (rules, my_ticket, tickets) = parse_input(input);
	let tickets: Vec<Ticket> = tickets.iter().cloned().filter(|t| find_invalid(t, &rules).len() == 0 ).collect();
	let mut known = vec![None; my_ticket.len()];

	let mut potentials = vec![];
	potentials.resize_with(my_ticket.len(), || rules.clone());

	// Limit rules that aren't possible
	potentials = potentials
					.iter()
					.enumerate()
					.map(|(i, p)| p.iter().cloned()
							.filter(|r| tickets.iter()
									.all(|t| r.check_value(& t[i])))
							.collect())
					.collect();

	while known.iter().any(|k| k.is_none()) {

		// Record where there is only one option as known
		for (i, p) in potentials.iter().enumerate() {
			if p.len() == 1 {
				known[i] = Some(p[0].field);

			}
		}

		potentials = potentials.iter().map(|p|
			p.iter().cloned().filter(|r| !known.contains(&Some(r.field))).collect()).collect();
		
	}
	
	known.iter().enumerate().filter(|(_i, k)| k.unwrap().starts_with("departure")).fold(1, |acc, (i,_k)| acc*my_ticket[i])
}

fn find_invalid(ticket: &Ticket, rules: &Vec<Rule>) -> Vec<u64> {
	ticket.iter().copied().filter(|v| !rules.iter().any(|r| r.check_value(v))).collect()
}

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
	let mut parts = input.split("\n\n");

	let rules = parse_rules(parts.next().unwrap());
	let my_ticket = parse_my_ticket(parts.next().unwrap());
	let tickets = parse_tickets(parts.next().unwrap());

	(rules, my_ticket, tickets)
}

fn parse_rules(input: &str) -> Vec<Rule> {
	input.lines().map(parse_rule).collect()
}

fn parse_rule(input: &str) -> Rule {
	let (name, ranges_str) = input.split_once(": ").unwrap();

	Rule {
		ranges: ranges_str.split(" or ").map(|rs| {
			let parts = rs.split_once('-').unwrap();
			u64::from_str_radix(parts.0, 10).unwrap()..=u64::from_str_radix(parts.1, 10).unwrap()
		}).collect(),
		field: name
	}
	
}

fn parse_my_ticket(input: &str) -> Ticket {
	parse_ticket(input.lines().nth(1).unwrap())
}

fn parse_tickets(input: &str) -> Vec<Ticket> {
	input.lines().skip(1).map(parse_ticket).collect()
}

fn parse_ticket(input: &str) -> Ticket {
	input.split(',').filter_map(|v| u64::from_str_radix(v, 10).ok()).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

		assert_eq!(part1(input), 71);
	}

		#[test]
	fn test_part2() {
		let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

		assert_eq!(part2(input), 0);
	}
}