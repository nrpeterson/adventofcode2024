use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated};
use adventofcode2024::build_main;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
    Invalid
}

use Instruction::*;

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse::<usize>())(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("mul("),
            terminated(
                separated_pair(number, char(','), number),
                char(')')
            )
        ),
        |(a, b)| Mul(a, b)
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt(
        (
            mul,
            map(tag("do()"), |_| Do),
            map(tag("don't()"), |_| Dont),
            map(anychar, |_| Invalid)
        )
    )(input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    many1(instruction)(input).unwrap().1
}

fn part1(input: &str) -> usize {
    parse_input(input).into_iter().filter_map(|p| {
        match p {
            Mul(a, b) => Some(a * b),
            _ => None
        }
    }).sum()
}

fn part2(input: &str) -> usize {
    parse_input(input).into_iter().fold(
        (0, true),
        |(total, is_enabled), instr| {
            match (instr, is_enabled) {
                (Mul(x, y), true) => (total + x * y, true),
                (Do, false) => (total, true),
                (Dont, true) => (total, false),
                _ => (total, is_enabled)
            }
        }
    ).0
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT1), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 48);
    }
}