#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::HashMap;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = count_orbits(&buffer);
    println!("Found {} orbits", n);

    Ok(())
}

fn build_map(data: &str) -> OrbitMap {
	data.lines().filter_map(|l| l.split_once(')')).map(|(p, c)| (c, p)).collect()
}

fn count_orbits(data: &str) -> u64 {
	let map = build_map(data);
	map.keys().map(|obj| count_orbits_for_obj(&map, obj)).sum()
}

fn count_orbits_for_obj(map: &OrbitMap, obj: &str) -> u64 {
	if obj == "COM" {
		return 0;
	}

	count_orbits_for_obj(map, map.get(obj).unwrap()	) + 1
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
}