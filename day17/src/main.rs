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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64
}

struct PointIter {
    min: Point,
    max: Point,
    current: Point
}

impl PointIter {
    fn new(world: &World) -> Self {
        let (min, max) = Self::world_min_max(&world);
        let current = Point {x: min.x-2, y: min.y-1, z: min.z-1, w: min.w-1};
        Self {
            min: Point {x: min.x-1, y: min.y-1, z: min.z-1, w: min.w-1},
            max: Point {x: max.x+1, y: max.y+1, z: max.z+1, w: max.w+1},
            current: current
        }
    }
    fn new_local(p: &Point) -> Self {
        let min = Point {x: p.x-1, y: p.y-1, z: p.z-1, w: p.w-1};
        let max = Point {x: p.x+1, y: p.y+1, z: p.z+1, w: p.w+1};
        let mut current = min.clone();
        current.x -= 1;
        Self {min: min, max: max, current: current}
    }

    fn world_min_max(world: &World) -> (Point, Point) {
        let mins = world.iter().fold(Point{x: 0, y: 0, z: 0, w: 0}, |m, p| Point {x: cmp::min(m.x, p.x), y: cmp::min(m.y, p.y), z: cmp::min(m.z, p.z), w: cmp::min(m.w, p.w) } );
        let maxs = world.iter().fold(Point{x: 0, y: 0, z: 0, w: 0}, |m, p| Point{x: cmp::max(m.x, p.x), y: cmp::max(m.y, p.y), z: cmp::max(m.z, p.z), w: cmp::max(m.w, p.w) });
        (mins, maxs)
    }
}

impl Iterator for PointIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.current.x + 1;
        let mut y = self.current.y;
        let mut z = self.current.z;
        let mut w = self.current.w;
        
        if x > self.max.x {
            x = self.min.x;
            y+=1;
            if y > self.max.y {
                y = self.min.y;
                z +=1;
                if z > self.max.z {
                    z = self.min.z;
                    w +=1;
                    if w > self.max.w {
                        return None
                    }
                }
            }
        }

        self.current = Point{x: x, y: y, z: z, w: w};
        Some(self.current.clone())
    }
}

type World = HashSet<Point>;

fn part1(input: &str) -> u64{
    (0..6).fold(build_world(input), |world, _| {
        PointIter::new(&world).filter(|point| {
            let neighbors = count_neighbors(&world, &point);
            (world.get(&point).is_some() && neighbors == 2) || neighbors == 3
        }).collect()
    }).len() as u64
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

fn count_neighbors(world: &World, point: &Point) -> u64 {
    PointIter::new_local(point).filter(|n| n != point && world.get(&n).is_some()).count() as u64
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

    #[test]
    fn test_iter() {
        let mut iter = PointIter::new_local(&Point {x: 0, y: 0, z: 0, w: 0});

        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: -1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 0, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 1, w: -1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 1, w: -1 }));

        //Inc W
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: -1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 0, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 1, w: 0 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 1, w: 0 }));

        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: -1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 0, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: -1, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: -1, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: -1, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 0, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 0, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 0, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: -1, y: 1, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 0, y: 1, z: 1, w: 1 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 1, z: 1, w: 1 }));

        assert_eq!(iter.next(), None);

    }
}