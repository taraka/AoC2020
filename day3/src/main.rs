use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = num_trees(&buffer);
    println!("Found {} trees", n);

    Ok(())
}

fn num_trees(forest: &str) -> u32 {
	forest.lines().enumerate().filter(|(y, row)| 
		row.chars().nth((y * 3) % row.chars().count()).unwrap() == '#' 
	).count() as u32
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