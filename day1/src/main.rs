use itertools::Itertools;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<u32> = buffer.lines()
                                .into_iter()
                                .filter_map(|x| u32::from_str_radix(x, 10).ok())
                                .collect();
    let output = do_stuff(input, 3, 2020);

    match output {
        Some(x) => println!("Found value: {}", x),
        None => println!("Not Found")
    }

    Ok(())
}


pub fn do_stuff(input: Vec<u32>, num_values: usize, find: u32) -> Option<u32> {
    Some(input.iter().combinations(num_values)
        .find(|x| x.iter().fold(0, |acc, j| acc + *j) == find)?
        .iter()
        .fold(1, |acc, v| acc * *v))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(do_stuff(vec![
            1721,
            979,
            366,
            299,
            116,
            2000,
            675,
            1456,
        ], 2, 2020), Some(514579));

        assert_eq!(do_stuff(vec![
            1721,
            979,
            366,
            300,
            116,
            2000,
            675,
            1456,
        ], 2, 2020).is_none(), true)
    }

    #[test]
    fn part2() {
        assert_eq!(do_stuff(vec![
            1721,
            979,
            366,
            299,
            116,
            2000,
            675,
            1456,
        ], 3, 2020), Some(241861950));
    }
}