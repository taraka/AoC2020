#![feature(str_split_once)]

use std::collections::HashMap;
use regex::Regex;
use std::io::{self, Read};

type Rule<'a> = Vec<Vec<&'a str>>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

    Ok(())
}
fn part1(input: &str) -> u64 {
	let (rules_str, messages) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);

    println!("{:?}", build_regex(&rules, 0));

	let m:Vec<_> = messages.lines().filter(|m| matches_zero(m, &rules)).collect();
	println!("{:?}", m);
	m.len() as u64
}

fn matches_zero(message: &str, rules: &HashMap<usize, Rule>) -> bool {
    let  regex = Regex::new(&format!("^{}$", build_regex(rules, 0))).unwrap();
    regex.is_match(message)
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
	input.lines()
		.filter_map(|l| l.split_once(": "))
		.map(|(is, rs)| (usize::from_str_radix(is, 10).unwrap(), parse_rule(rs)))
		.collect()
} 

fn parse_rule(input: &str) -> Rule {
	input.split(" | ").map(|p| p.split(' ').collect()).collect()
}

fn build_regex(rules: &HashMap<usize, Rule>, id: usize) -> String {
	if rules[&id][0][0].chars().nth(0).unwrap() == '"' {
		rules[&id][0][0].trim_matches('"').to_owned()
	}
	else {
		format!("({})", rules[&id].iter().map(|p| p.iter().filter_map(|r| usize::from_str_radix(r,10).ok()).map( |r| build_regex(rules, r) ).collect::<Vec<String>>().join("")  ).collect::<Vec<String>>().join("|"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

		assert_eq!(part1(input), 2);
	}
}