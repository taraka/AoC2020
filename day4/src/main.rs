#![feature(str_split_once)]

use std::io::{self, Read};
use std::collections::{HashMap, HashSet};
use regex::Regex;
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
	let fields: HashMap<_,_> = passport.split_whitespace()
										.filter_map(|d| d.split_once(':'))
										.collect();
	
	if !REQUIRED.iter().all(|r| fields.keys().copied().collect::<HashSet<&str>>().contains(r)) {
		return false
	}

	check_byr(fields["byr"]) &&
		check_iyr(fields["iyr"]) &&
		check_eyr(fields["eyr"]) &&
		check_hgt(fields["hgt"]) &&
		check_hcl(fields["hcl"]) &&
		check_ecl(fields["ecl"]) &&
		check_pid(fields["pid"])
	
}

fn check_byr(value: &str) -> bool {
	check_year(value, 1920, 2002)
}

fn check_iyr(value: &str) -> bool {
	check_year(value, 2010, 2020)
}

fn check_eyr(value: &str) -> bool {
	check_year(value, 2020, 2030)
}

fn check_year(value: &str, min: u16, max: u16) -> bool {
	match u16::from_str_radix(value, 10) {
		Ok(x) => x >= min && x <= max,
		_ => false
	}
}

fn check_hgt(value: &str) -> bool {
	lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(?P<num>\d+)(?P<unit>cm|in)$"
        ).unwrap();
    }

    RE.captures(value).map(|caps| {
    	let num = u16::from_str_radix(&caps["num"], 10).unwrap();

    	match &caps["unit"] {
    		"in" => num >= 59 && num <= 76,
			"cm" => num >= 150 && num <= 193,
    		_ => false
    	}
    }).unwrap_or(false)
}

fn check_hcl(value: &str) -> bool {
	lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^#[\da-f]{6}$"
        ).unwrap();
    }

    RE.is_match(value)
}

fn check_ecl(value: &str) -> bool {
	lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(amb|blu|brn|gry|grn|hzl|oth)$"
        ).unwrap();
    }

    RE.is_match(value)
	
}

fn check_pid(value: &str) -> bool {
	lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^\d{9}$"
        ).unwrap();
    }

    RE.is_match(value)
	
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

    #[test]
    fn all_invalid() {
    	let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

 	assert_eq!(check_passports(input), 0);
    }

    #[test]
    fn all_valid() {
    	let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

 	assert_eq!(check_passports(input), 4);
    }

}