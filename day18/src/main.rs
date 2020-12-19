use std::io::{Read, self};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part2(&buffer);
    println!("Part2: {}", n);

    Ok(())
}

fn part2(input: &str) -> i64 {
	input.lines().inspect(|l| println!("{}\n{}", l, parser::eval(l).unwrap()))
	.map(|l| parser::eval(l).unwrap())
	.sum()
}

peg::parser! {
    grammar parser() for str {
        pub rule eval() -> i64 = precedence!{
        		a:(@) _ "*" _ b:@ { a * b }
        		--
                a:(@) _ "+" _ b:@ { a + b }
				--                
                "(" _ e:eval() _ ")" { e }
                n:number() { n }
        }

        rule number() -> i64
            = n:$(['0'..='9']+) { i64::from_str_radix(n, 10).unwrap() }

        rule _() = " "?
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		assert_eq!(parser::eval("3 + 2").unwrap(), 5);
		assert_eq!(parser::eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(), 23340);
		assert_eq!(parser::eval("4 * 2 + 3 * ((9 * 3 * 8 * 3 * 7 * 2) + 6 * 3 * (9 * 7 * 9 + 6 * 5) * 4 + 7) + 3 + (2 + 9 * 3 * 9 + (7 + 7 * 3 + 4))").unwrap(), 23340);
		
	}
}