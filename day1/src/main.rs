use std::io::{self, Read};


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<u32> = buffer.lines()
    							.into_iter()
    							.filter_map(|x| u32::from_str_radix(x, 10).ok())
    							.collect();
   	let output = do_stuff(input);

   	match output {
		Some(x) => println!("Found value: {}", x),
		None => println!("Not Found")
   	}

    Ok(())
}


pub fn do_stuff(input: Vec<u32>) -> Option<u32> {
	let mut list = input.clone();
	list.sort_unstable();

	for i in 0..list.len() {
		let low = list[i];
		for high in list[i..].iter().rev() {
			let sum = high + low;

			if sum == 2020 {
				return Some(high * low);
			}
		}
	}

	None
}



#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn does_stuff() {
        assert_eq!(do_stuff(vec![
        	1721,
			979,
			366,
			299,
			116,
			2000,
			675,
			1456,
        ]), Some(514579));

        assert_eq!(do_stuff(vec![
        	1721,
			979,
			366,
			300,
			116,
			2000,
			675,
			1456,
        ]).is_none(), true)
    }
}