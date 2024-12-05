use itertools::Itertools;
use nom::{IResult};
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2024::build_main;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let num = || map_res(digit1, |d: &str| d.parse::<usize>());
    let pair = separated_pair(num(), space1, num());
    let result: IResult<&str, Vec<(usize, usize)>> = separated_list1(newline, pair)(input);
    result.unwrap().1.into_iter().unzip()
}

fn part1(input: &str) -> usize {
    let (mut l, mut r) = parse_input(input);
    l.sort();
    r.sort();

    l.into_iter().zip(r.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn condensed(v: Vec<usize>) -> impl Iterator<Item=(usize, usize)> {
    v.into_iter().map(|c| (c, 1))
        .coalesce(|(a, a_count), (b, b_count)| {
            if a == b { Ok((a, a_count + b_count)) } else { Err(((a, a_count), (b, b_count))) }
        })
}

fn part2(input: &str) -> usize {
    let (mut l, mut r) = parse_input(input);
    l.sort();
    r.sort();

    let mut l_merged = condensed(l);
    let mut r_merged = condensed(r);

    let mut l_cur = l_merged.next();
    let mut r_cur = r_merged.next();

    let mut result = 0;

    while let (Some((x, x_count)), Some((y, y_count))) = (l_cur, r_cur) {
        if x < y {
            l_cur = l_merged.next();
        }
        else if x > y {
            r_cur = r_merged.next();
        }
        else {
            result += x * x_count * y_count;
            l_cur = l_merged.next();
            r_cur = r_merged.next();
        }

    }

    result
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(INPUT), 31);
    }
}
