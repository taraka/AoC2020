use std::io::{self, Read};

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

    Ok(())
}

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

#[derive(Debug)]
struct Ship {
    n: i64,
    w: i64,
    facing: i64
}

impl Ship {
	fn new() -> Self {
		Self {
			n: 0,
			w: 0,
			facing: 0
		}
	}

	fn nav(self, dir: &Dir, value: i64) -> Self {
		match dir {
			Dir::North => Ship { n: self.n + value, w: self.w, facing: self.facing },
			Dir::South => Ship { n: self.n - value, w: self.w, facing: self.facing },
			Dir::East => Ship { n: self.n, w: self.w - value, facing: self.facing },
			Dir::West => Ship { n: self.n, w: self.w + value, facing: self.facing },
			Dir::Forward => { 
				let dir = get_dir(self.facing);
				self.nav(&dir, value)
			},
			Dir::Right => Ship { n: self.n, w: self.w, facing: (self.facing + value) % 360},
			Dir::Left => Ship { n: self.n, w: self.w, facing: (self.facing - value + 360) % 360},
		}
	}
}

fn get_dir(facing: i64) -> Dir {

	match facing {
		0 => Dir::East,
		90 => Dir::South,
		180 => Dir::West,
		270 => Dir::North,
		_ => panic!("We shouldn't be pointing this way, {}", facing)
	}
}

fn parse_input(input: &str) -> Vec<(Dir, i64)> {
	input.lines().map(|l| (
		match l.chars().nth(0).unwrap() {
			'N' => Dir::North,
			'S' => Dir::South,
			'E' => Dir::East,
			'W' => Dir::West,
			'L' => Dir::Left,
			'R' => Dir::Right,
			'F' => Dir::Forward,
			_ => panic!("Unknown direction")
		}
	, i64::from_str_radix(&l[1..], 10).unwrap() ))
	.collect()
}

fn get_distance(ship: Ship) -> i64 {
	ship.w.abs() + ship.n.abs()
}

fn part1(input: &str) -> i64 {
	let instructions = parse_input(input);
	let ship = instructions.iter().fold(Ship::new(), |s, (d, v)| s.nav(d, *v));

	get_distance(ship)
}


#[cfg(test)]
mod tests {
		use super::*;

	#[test]
	fn test_part1() {
		let input = r#"F10
N3
F7
R90
F11"#;
	assert_eq!(part1(input), 25);
	}
}