#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::HashMap;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = count_orbits(&buffer);
    println!("Found {} orbits", n);

    let d = find_route(&buffer);
    println!("Distance is {}", d);

    Ok(())
}

fn build_map(data: &str) -> OrbitMap {
	data.lines().filter_map(|l| l.split_once(')')).map(|(p, c)| (c, p)).collect()
}

fn count_orbits(data: &str) -> u64 {
	let map = build_map(data);
	map.keys().map(|obj| find_parents(&map, obj).len() as u64).sum()
}

fn find_route(data: &str) -> u64 {
	let map = build_map(data);
	let you = find_parents(&map, "YOU");
	let san = find_parents(&map, "SAN");
	let common: Vec<&str> = you.iter().filter(|y| san.contains(y)).copied().collect();

	(you.len() + san.len() - (common.len() * 2) - 2) as u64
}

fn find_parents<'a>(map: &'a OrbitMap, obj: &'a str) -> Vec<&'a str> {
	if obj == "COM" {
		return Vec::new();
	}

	let mut parents = find_parents(map, map.get(obj).unwrap());
	parents.push(obj);
	parents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_stuff() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        assert_eq!(count_orbits(input), 42);

    }


    #[test]
    fn part2() {
    	let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

	assert_eq!(find_route(input), 4);
    }
}