use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = check_slopes(&buffer, vec![(1,1), (3,1), (5,1), (7,1), (1,2)]);
    println!("Found {} trees", n);

    Ok(())
}

fn check_slopes(forest: &str, slopes: Vec<(usize, usize)>) -> u64 {
	slopes.iter().map(|(x, y)| num_trees(forest, *x, *y))
		.fold(1, |acc, v| acc * v)
}

fn num_trees(forest: &str, x_step: usize, y_step: usize) -> u64 {
	forest.lines().enumerate()
	.step_by(y_step)
	.filter(|(y, row)| 
		row.chars().nth(((y/y_step) * x_step) % row.chars().count()).unwrap() == '#' 
	).count() as u64
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

        assert_eq!(num_trees(input, 3, 1), 7);
        assert_eq!(check_slopes(input, vec![(1,1), (3,1), (5,1), (7,1), (1,2)]), 336);

    }
}