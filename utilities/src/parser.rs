// utilities/src/lib.rs

// dependencies
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::{preceded, terminated};
use nom::IResult;

// parser to look for the word "elf"
fn elf(input: &str) -> IResult<&str, &str> {
    tag("elf")(input)
}

// parser to look for the word "shelf"
fn shelf(input: &str) -> IResult<&str, &str> {
    tag("shelf")(input)
}

// parser to look for the phrase "elf on a"
fn elf_on_a(input: &str) -> IResult<&str, &str> {
    tag("elf on a ")(input)
}

// parser to look for the phrase "elf on a shelf"
fn elf_on_a_shelf(input: &str) -> IResult<&str, &str> {
    preceded(elf_on_a, shelf)(input)
}

// parser to look for instances of the word "shelf" that are preceded by the phrase "elf on a"
fn shelf_no_elf(input: &str) -> IResult<&str, &str> {
    terminated(elf_on_a, shelf)(input)
}

// combinator to parse the input string for instances of the word "elf"
pub fn parse_elf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf))(input)
}

// combinator to parse the input string for instances of the phrase "elf on a shelf"
pub fn parse_elf_on_a_shelf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf_on_a_shelf))(input)
}

// combinator to parse the input string for instances of the word "shelf" that are preceded by the phrase "elf on a"
pub fn parse_shelves_with_no_elves(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), shelf_no_elf))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elf() {
        let input = "The mischievous elf peeked out from behind the toy workshop, and another elf joined in the festive dance. Look, there is also an elf on that shelf!";
        let elves = parse_elf(input).unwrap();
        let elf_count = elves.1.len();
        assert_eq!(elf_count, 4);
    }

    #[test]
    fn test_parse_elf_on_a_shelf() {
        let input = "there is an elf on a shelf on an elf. there is also another shelf in Belfast.";
        let elves = parse_elf_on_a_shelf(input).unwrap();
        let elf_count = elves.1.len();
        assert_eq!(elf_count, 1);
    }

    #[test]
    fn test_parse_shelves_with_no_elves() {
        let input = "there is an elf on a shelf on an elf. there is also another shelf in Belfast.";
        let shelves = parse_shelves_with_no_elves(input).unwrap();
        let shelf_count = shelves.1.len();
        assert_eq!(shelf_count, 1);
    }
}
