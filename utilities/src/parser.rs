// utilities/src/lib.rs

// dependencies
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

fn elf(input: &str) -> IResult<&str, &str> {
    tag("elf")(input)
}

fn shelf(input: &str) -> IResult<&str, &str> {
    tag("shelf")(input)
}

fn elf_on_a(input: &str) -> IResult<&str, &str> {
    tag("elf on a ")(input)
}

fn elf_on_a_shelf(input: &str) -> IResult<&str, &str> {
    preceded(elf_on_a, shelf)(input)
}

pub fn parse_elf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf))(input)
}

pub fn parse_elf_on_a_shelf(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(take(1usize), elf_on_a_shelf))(input)
}

pub fn parse_shelves_with_no_elves(input: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    todo!()
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
