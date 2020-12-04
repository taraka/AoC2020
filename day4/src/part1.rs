use std::io::{self, Read};
use std::collections::HashSet;
use lazy_static::lazy_static;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = check_passports(&buffer);
    println!("Found {} passports", n);

    Ok(())
}

fn check_passports(input: &str) -> u64 {
	input.split("\n\n").filter(|p| is_valid(*p)).count() as u64
}


fn is_valid(passport: &str) -> bool {
    lazy_static! {
        static ref REQUIRED: HashSet<&'static str> = [ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ].iter().cloned().collect();
    }
	let fields: HashSet<&str> = passport.split_whitespace()
										.map(|d| d.split(':').nth(0).unwrap())
										.collect();

	REQUIRED.iter().all(|r| fields.contains(r))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_stuff() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;


        assert_eq!(check_passports(input), 2);

    }
}