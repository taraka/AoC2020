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
	let  regex = Regex::new(&format!("^{}$", build_regex(&rules, 0, 0, 14))).unwrap();

	let m:Vec<_> = messages.lines().filter(|m| regex.is_match(m)).collect();
	m.len() as u64
}

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
	input.lines()
		.filter_map(|l| l.split_once(": "))
		.map(|(is, rs)| (is, match is {
			"8" => "42 | 42 8",
			"11" => "42 31 | 42 11 31",
			_ => rs
		}))
		.map(|(is, rs)| (usize::from_str_radix(is, 10).unwrap(), parse_rule(rs)))
		.collect()
} 

fn parse_rule(input: &str) -> Rule {
	input.split(" | ").map(|p| p.split(' ').collect()).collect()
}

fn build_regex(rules: &HashMap<usize, Rule>, id: usize, depth: u8, stop_depth: u8) -> String {
	if depth > stop_depth {
		"d".to_owned()
	}
	else if rules[&id][0][0].chars().nth(0).unwrap() == '"' {
		rules[&id][0][0].trim_matches('"').to_owned()
	}
	else {
		format!("({})", rules[&id].iter()
							.map(|p| p.iter()
									.filter_map(|r| usize::from_str_radix(r,10).ok())
									.map( |r| build_regex(rules, r, depth + 1, stop_depth) )
									.collect::<Vec<String>>().join(""))
							.collect::<Vec<String>>().join("|"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

		assert_eq!(part1(input), 12);
	}
}