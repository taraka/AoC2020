use std::io::{Read, self};
use std::collections::HashSet;
use std::cmp;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    let n = part1(&buffer);
    println!("Part1: {}", n);

    // let n = part2(&input);
    // println!("Part2: {}", n);

    Ok(())
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Point {
	x: i64,
	y: i64,
	z: i64,
	w: i64
}

type World = HashSet<Point>;

fn part1(input: &str) -> u64{
	let mut world = build_world(input);

	for _ in 0..6 {
		let (min, max) = world_min_max(&world);
		let mut new_world = World::new();

		for x in (min.x-1)..=(max.x+1) {
			for y in (min.y-1)..=(max.y+1) {
				for z in (min.z-1)..=(max.z+1) {
					for w in (min.w-1)..=(max.w+1) {
						let point = Point {x: x, y: y, z: z, w: w};
						let neighbors = count_neighbors(&world, &point);

						if (world.get(&point).is_some() && neighbors == 2) || neighbors == 3 {
							new_world.insert(point);
						}
					}
				}	
			}		
		}

		world = new_world;
	}

	world.len() as u64
}

fn build_world(input: &str) -> World {
	let mut world = World::new();
	input
		.lines()
		.enumerate()
		.for_each(|(y, line)| line
					.chars()
					.enumerate()
					.filter(|(_x, c)| *c == '#')
					.for_each(|(x, _c)| { 
						world.insert(Point{x: x as i64, y: y as i64, z: 0, w: 0});
					}));
	world 
}

fn world_min_max(world: &World) -> (Point, Point) {
	let mins = world.iter().fold(Point{x: 0, y: 0, z: 0, w: 0}, |m, p| Point {x: cmp::min(m.x, p.x), y: cmp::min(m.y, p.y), z: cmp::min(m.z, p.z), w: cmp::min(m.w, p.w) } );
	let maxs = world.iter().fold(Point{x: 0, y: 0, z: 0, w: 0}, |m, p| Point{x: cmp::max(m.x, p.x), y: cmp::max(m.y, p.y), z: cmp::max(m.z, p.z), w: cmp::max(m.w, p.w) });
	(mins, maxs)
}

fn count_neighbors(world: &World, point: &Point) -> u64 {
	let mut count = 0;

	for x in (point.x-1)..=(point.x+1) {
		for y in (point.y-1)..=(point.y+1) {
			for z in (point.z-1)..=(point.z+1) {
				for w in (point.w-1)..=(point.w+1) {
					let n = Point {x: x, y: y, z: z, w: w};
					if n != *point && world.get(&n).is_some() {
						count += 1;
					}
				}
			} 	
		} 
	}

	count
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(part1(r#".#.
..#
###"#), 848);
	}
}