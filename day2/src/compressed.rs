use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"^(?P<low>\d+)-(?P<high>\d+) (?P<c>[a-zA-Z]): (?P<password>.*)$").unwrap();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = buffer.lines()
            .into_iter()
            .filter_map(|i| {
                let caps = re.captures(i)?;
                let n = caps["password"].chars().filter(|v|  v == &caps["c"].chars().nth(0).unwrap()).count() as u32;
                if n >= u32::from_str_radix(&caps["low"], 10).unwrap() && n <= u32::from_str_radix(&caps["high"], 10).unwrap() {Some(1)} else {None}
            })
            .count();

    println!("Found {} valid passwords", n);

    Ok(())
}