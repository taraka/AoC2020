use std::io::{self, Read};
use std::collections::HashSet;

type Person = HashSet<char>;
type Group = Vec<Person>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part 1 {}", sum_counts(&buffer));
    println!("Part 2 {}", count_everyone(&buffer));

    Ok(())
}

fn sum_counts(data: &str) -> u64 {
    data.split("\n\n")
        .map(|g| g.chars()
                    .filter(|c| c.is_ascii_lowercase())
        .collect::<Person>().len() as u64)
        .sum()
}

fn count_everyone(data: &str) -> u64 {
    data.split("\n\n")
        .map(parse_group)
        .map(count_everyone_group)
        .sum()
}

fn parse_group(g: &str) -> Group {
    g.split('\n')
        .map(parse_person)
        .filter(|g| !g.is_empty())
        .collect()
}

fn parse_person(p: &str) -> Person {
    p.chars()
        .filter(char::is_ascii_lowercase)
        .collect()
            
}

fn count_everyone_group(group: Group) -> u64 {
    (b'a' ..= b'z')
        .map(char::from)
        .map(|c| (c, group.iter().filter(|p| p.contains(&c)).count()))
        .filter(|(_c, n)| n == &group.len())
        .map(|(c, _n)| c)
        .count() as u64
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
        assert_eq!(count_everyone(input), 7);

    }
}