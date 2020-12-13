fn main() {
    let earliest = 1002578;
        let buses: Vec<Option<u64>> = "19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,751,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,431,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,17"
                    .split(',')
                    .map(|v| u64::from_str_radix(v, 10).ok())
                    .collect();

        println!("Part1: {}", part1(earliest, buses.iter().filter_map(|v| *v).collect() ));
        println!("Part2: {}", part2(buses));
}

fn part1(earliest: u64, buses: Vec<u64>) -> u64 {
    let mut ts = earliest; 

    loop {
        match buses.iter().find(|b| ts % *b == 0) {
            Some(b) => return *b * (ts - earliest),
            None => ts += 1
        }
    }
}

fn part2(buses: Vec<Option<u64>>) -> u64 {
    let mut t = 0; 
    let bs: Vec<(u64, u64)> = buses.iter().enumerate().filter_map(|(to, b)| Some((to as u64, (*b)?))).collect();
    let first = buses.iter().nth(0).unwrap().unwrap();
    while !bs.iter().all(|(to, b)| (t + *to) % b == 0) {
        if t % 10000 * first == 0 {
            print!("Working on {}\r", t);
        }
        t += first;
    }

    t
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let earliest = 939;
        let buses = "7,13,x,x,59,x,31,19"
                    .split(',')
                    .filter_map(|v| u64::from_str_radix(v, 10).ok())
                    .collect();

        assert_eq!(part1(earliest, buses), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(p2("17,x,13,19")), 3417);
        assert_eq!(part2(p2("67,7,59,61")), 754018);
        assert_eq!(part2(p2("67,x,7,59,61")), 779210);
        assert_eq!(part2(p2("67,7,x,59,61")), 1261476);
        assert_eq!(part2(p2("7,13,x,x,59,x,31,19")), 1068781);
    }

    fn p2(s: &str) -> Vec<Option<u64>> {
        s.split(',')
            .map(|v| u64::from_str_radix(v, 10).ok())
            .collect()
    }
}