use std::io::{Read, self};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let n = part2(&buffer);
    println!("Part2: {}", n);

    Ok(())
}

fn part2(input: &str) -> i64 {
	input.lines().map(|l| parser::eval(l).unwrap()).sum()
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
		assert_eq!(parser::eval("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
		
	}
}