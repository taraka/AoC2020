#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

type Bag<'a> = &'a str;
type Holds<'a> = Vec<(u64, Bag<'a>)>;


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = count_eventually_contains(&buffer,"shiny gold");
    println!("Found {} bags", n);

    Ok(())
}

fn build_rules(input: &str) -> HashMap<Bag, Holds> {
	input.lines().map(parse_bag_line).collect()
}

fn parse_bag_line(line: &str) -> (Bag, Holds) {
	let (bag, holds_str) = line.trim_end_matches('.').split_once(" contain ").unwrap();
	let holds = holds_str.split(", ")
        .filter_map(|l| l.split_once(' '))
        .map(|(n, hold)| (u64::from_str_radix(n, 10).unwrap_or(0), parse_bag(hold)))
        .collect();

    (parse_bag(bag), holds)
}

fn parse_bag(line: &str) -> Bag {
    line.trim_end_matches('s').trim_end_matches(" bag")
}


fn count_eventually_contains(input: &str, obj: &str) -> u64 {
	let rules = build_rules(input);
    
    (count_eventually_contains_rec(&rules, obj).len() - 1) as u64
}

fn count_eventually_contains_rec<'a>(rules: &'a HashMap<Bag, Holds>, obj: Bag<'a>) -> HashSet<Bag<'a>> {
    let mut set: HashSet<Bag> = rules.iter()
        .filter(|(_b, r)| r.iter()
                            .map(|t| t.1)
                            .any(|b| b == obj))
        .flat_map(|(b, _r)| count_eventually_contains_rec(rules, b))
        .collect();

    set.insert(obj);

    return set;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;


        assert_eq!(count_eventually_contains(input, "shiny gold"), 4);

    }


}