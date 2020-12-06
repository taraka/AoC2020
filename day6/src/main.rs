#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = sum_counts(&buffer);
    println!("Answer {}", n);


    Ok(())
}

fn sum_counts(data: &str) -> u64 {
	data.split("\n\n").map(|g| g.chars().filter(|c| c.is_ascii_lowercase()).collect::<HashSet<char>>().len() as u64).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_stuff() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(sum_counts(input), 11);

    }
}