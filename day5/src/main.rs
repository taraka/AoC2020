use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let max = buffer.lines().into_iter().filter_map(seat_id).max();

    println!("Max ID is: {}", max.unwrap());

    Ok(())
}

fn get_number(s: &str, high: char, _low: char) -> Option<u64> {
	u64::from_str_radix(&s.chars().map(|c| if c == high {'1'} else {'0'}).collect::<String>(), 2).ok()
}

fn row_num (s: &str) -> Option<u64> {
	get_number(s, 'B', 'F')
}

fn col_num (s: &str) -> Option<u64> {
	get_number(s, 'R', 'L')
}

fn seat_id(seat: &str) -> Option<u64> {
	Some(row_num(&seat[..7])? * 8 + col_num(&seat[7..])?)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
    	assert_eq!(seat_id("BFFFBBFRRR"), Some(567));
    	assert_eq!(seat_id("FFFBBBFRRR"), Some(119));
    	assert_eq!(seat_id("BBFFBBFRLL"), Some(820));
    }
}