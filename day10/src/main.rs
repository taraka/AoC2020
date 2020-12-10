use std::io::{self, Read};

fn main() -> io::Result<()>  {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut input: Vec<u64> = buffer.lines()
                                .into_iter()
                                .filter_map(|x| u64::from_str_radix(x, 10).ok())
                                .collect();
    input.sort();

    let n = part1(&input);
    println!("Part1: {}", n);

    let n = part2(&input);
    println!("Part2: {}", n);

    Ok(())
}


fn part1(input: &Vec<u64>) -> u64 {
    let mut prev: u64 = 0;
    let mut counts: [u64; 3] = [0, 0, 1];

    for v in input.iter() {
        counts[(v - prev - 1) as usize] += 1;
        prev = *v;
    }
    counts[0] * counts[2]
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut counts = vec![0; (input[input.len() - 1] as usize) + 1];

    for &v in input.iter() {
        if v <= 3 {
            counts[v as usize] += 1;

            for x in (1..v as usize).rev() {
                counts[v as usize] += counts[x];
            }

            continue;
        }

        counts[v as usize] += counts[v as usize - 3];
        counts[v as usize] += counts[v as usize - 2];
        counts[v as usize] += counts[v as usize - 1];
    }

    counts.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part1_full() {
        let mut input = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        input.sort();
        assert_eq!(part1(&input), 220);

    }

    #[test]
    fn test_part2() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(part2(&input), 8);
    }

    // #[test]
    fn test_part2_full() {
        let mut input = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        input.sort();
        assert_eq!(part2(&input), 19208);

    }


}