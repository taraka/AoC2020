use std::io::{self, Read};

type Grid = Vec<Vec<Slot>>;

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part1(&buffer);
    println!("Part1: {}", n);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Slot {
	FLOOR,
	EMPTY,
	OCCUPIED
}

fn pg(grid: &Grid) -> String {
	grid.iter().map(|r| {
		let mut rs = r.iter().map(|s| match s {
			Slot::FLOOR => '.',
			Slot::EMPTY => 'L',
			Slot::OCCUPIED => '#'
		}).collect::<String>();
		rs.push('\n');
		rs
	}).collect()
}

fn count_around(grid: &Grid, tx: usize, ty: usize) -> u8 {
	let height = grid.len();
	let width = grid[0].len();

	let mut count = 0;

	for x in tx.saturating_sub(1)..=tx+1 {
		for y in ty.saturating_sub(1)..=ty+1 {
			
			if (0..width).contains(&x) &&
					(0..height).contains(&y) &&
					!(y == ty && x == tx) &&
					grid[y][x] == Slot::OCCUPIED {
				count += 1;
			}
		}
	}

	count
}

fn count_can_see(grid: &Grid, tx: usize, ty: usize) -> u8 {
	let mut count = 0;
	for dx in -1..=1 {
		for dy in -1..=1 {
			if dx==0 && dy==0 {
				continue;
			}
			let seat = find_seat_dir(grid, tx, ty, dx, dy);
			
			if seat == Slot::OCCUPIED {
				count += 1;
			}
		}
	}

	count
}

fn find_seat_dir(grid: &Grid, tx: usize, ty: usize, dx: isize, dy: isize) -> Slot {
	let mut i = 1;
	loop {
		if let Some(row) = grid.get((dy*i).saturating_add(ty as isize) as usize) {
			match row.get((dx*i).saturating_add(tx as isize) as usize) {
				Some(Slot::OCCUPIED) => return Slot::OCCUPIED,
				Some(Slot::EMPTY) => return Slot::EMPTY,
				Some(Slot::FLOOR) => (),
				None => return Slot::FLOOR,
			}
		}
		else {
			return Slot::FLOOR;
		}
		i += 1;
	}
}

fn part1(input: &str) -> u64 {
	let mut grid: Grid = input.lines().map(parse_line).collect();
	let mut gen_id = 0;
	loop {
		println!("\n{}\n", pg(&grid));

		let (success, new_grid) = generation(&grid);
		grid = new_grid;
		if success {
			break;
		}
		gen_id += 1;
	}
	
	count_occupied(&grid)
}

fn parse_line(line: &str) -> Vec<Slot> {
	line.chars().filter_map(|c| match c {
		'L' => Some(Slot::EMPTY),
		'.' => Some(Slot::FLOOR),
		'#' => Some(Slot::OCCUPIED),
		_ => None
	}).collect()
}

fn generation(current: &Grid) -> (bool, Grid) {
	let mut next: Grid = vec![vec![]];

	for (y, l) in current.iter().enumerate() {
		for (x, s) in l.iter().enumerate() {
			let count = count_can_see(&current, x, y);
			
			let new_slot = match s {
				Slot::FLOOR => Slot::FLOOR,
				Slot::EMPTY => if count == 0 {Slot::OCCUPIED} else {Slot::EMPTY},
				Slot::OCCUPIED => if count >= 5 {Slot::EMPTY} else {Slot::OCCUPIED}
			};

			if next.get(y).is_none() {
				next.insert(y, vec![]);
			}

			next[y].insert(x, new_slot);
		}
	}

	(&next == current, next)
}

fn count_occupied(current: &Grid) -> u64 {
	current.iter().map(|row| row.iter().filter(|s| *s == &Slot::OCCUPIED).count() as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

	#[cfg(test)]
    fn test_part2() {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;


	
assert_eq!(count_can_see(&vec![
        	vec![Slot::EMPTY, Slot::FLOOR, Slot::OCCUPIED],
        	vec![Slot::OCCUPIED, Slot::OCCUPIED, Slot::OCCUPIED],
        	], 0, 0), 3); 
        assert_eq!(part1(input), 26);

    }

}