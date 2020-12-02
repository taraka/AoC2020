use std::io::{self, Read};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Password {
    range: std::ops::Range<u32>,
    c: char,
    password: String
}

impl Password {
    fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?P<low>\d+)-(?P<high>\d+) (?P<c>[a-zA-Z]): (?P<password>.*)$"
            ).unwrap();
        }
        
        RE.captures(s).map(|caps|
            Password {
                range: (u32::from_str_radix(&caps["low"], 10).unwrap())..(u32::from_str_radix(&caps["high"], 10).unwrap()+1),
                c: caps["c"].chars().nth(0).unwrap(),
                password: caps["password"].to_owned()
            })
            
    }

    fn is_valid(&self) -> bool {
        let n = self.password.chars().filter(|v| v == &self.c).count() as u32;
        self.range.contains(&n)
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = num_valid(&buffer);
    println!("Found {} valid passwords", n);

    Ok(())
}


fn num_valid(input: &str) -> u64 {
        input.lines()
            .into_iter()
            .filter_map(Password::from_str)
            .filter(|p|  p.is_valid())
            .count() as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_stuff() {
        let input = r#"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
        "#;

        assert_eq!(num_valid(input), 2);
    }
}