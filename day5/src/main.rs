use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let max = buffer.lines().into_iter().filter_map(seat_id).max();
    println!("Max ID is: {}", max.unwrap());

    let mut seats: Vec<u64> = buffer.lines().into_iter().filter_map(seat_id).collect();
	seats.sort();
    
    let mut prev: &u64 = &0;
	for id in seats.iter() {
		if id - prev == 2 {
			println!("ID: {}", id-1);
		}
		prev = id;
	} 

    Ok(())
}


fn seat_id(seat: &str) -> Option<u64> {
	u64::from_str_radix(&seat.chars().map(|c| if vec!['R', 'B'].contains(&c) {'1'} else {'0'}).collect::<String>(), 2).ok()
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