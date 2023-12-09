// utilities/src/lib.rs

// dependencies
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::multi::many0;
use nom::multi::many_till;
use nom::IResult;

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), tag("elf")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "The mischievous elf peeked out from behind the toy workshop, and another elf joined in the festive dance. Look, there is also an elf on that shelf! Another elf could be seen, creeping about in the shadows.";
        let expected = parse_input(input).unwrap();
        let count = expected.1.len();
        assert_eq!(count, 5);
    }
}