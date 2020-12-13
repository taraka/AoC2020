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
    e: i64,
    wn: i64,
    we: i64
}

impl Ship {
	fn new() -> Self {
		Self { n: 0, e: 0, wn: 1, we: 10 }
	}

	fn nav(self, dir: &Dir, value: i64) -> Self {
		match dir {
			Dir::North => Ship { n: self.n, e: self.e, wn: self.wn + value, we: self.we },
			Dir::South => Ship { n: self.n, e: self.e, wn: self.wn - value, we: self.we },
			Dir::East => Ship { n: self.n, e: self.e, wn: self.wn, we: self.we + value },
			Dir::West => Ship { n: self.n, e: self.e, wn: self.wn, we: self.we - value },
			Dir::Forward => Ship { n: self.n + (self.wn * value), e: self.e + (self.we * value), wn: self.wn, we: self.we },
			Dir::Right => rotate_waypoint(self, get_rotation(dir, value)),
			Dir::Left => rotate_waypoint(self, get_rotation(dir, value)),
		}
	}
}

fn rotate_waypoint(ship: Ship, rot: i64) -> Ship {
	match rot {
		90 => Ship {n: ship.n, e: ship.e, wn: 0 - ship.we, we: ship.wn},
		180 => Ship {n: ship.n, e: ship.e, wn: 0 - ship.wn,  we:  0 - ship.we},
		270 => Ship {n: ship.n, e: ship.e, wn: ship.we, we: 0 - ship.wn},
		_ => panic!("Don't know how to do that")
	}
}

fn get_rotation(dir: &Dir, value: i64) -> i64 {
	match dir {
		Dir::Left => 360 - value,
		Dir::Right => value,
		_ => panic!("Woops")
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
	, i64::from_str_radix(&l[1..], 10).unwrap()) )
	.collect()
}

fn get_distance(ship: Ship) -> i64 {
	ship.e.abs() + ship.n.abs()
}

fn part1(input: &str) -> i64 {
	let instructions = parse_input(input);
	let ship = instructions.iter().fold(Ship::new(), |s, (d, v)| s.nav(d, *v));
	println!("{:?}", ship);
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
	assert_eq!(part1(input), 286);
	}


	#[test]
	fn test_part1_l() {
		let input = r#"F10
N3
F7
L270
F11"#;
	assert_eq!(part1(input), 286);
	}
}

//Not 27525