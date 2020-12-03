use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = num_trees(&buffer);
    println!("Found {} trees", n);

    Ok(())
}

fn num_trees(forest: &str) -> u32 {
	let mut count = 0;
	let mut y = 0;
	for row in forest.lines() {
		let x = (y * 3) % row.chars().count();
		if row.chars().nth(x).unwrap() == '#' {
			count += 1;
		}
		y += 1;
	}
	count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_stuff() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        assert_eq!(num_trees(input), 7);
    }
}