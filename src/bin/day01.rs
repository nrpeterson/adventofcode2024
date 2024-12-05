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

fn part2(input: &str) -> usize {
    let (mut l, mut r) = parse_input(input);
    l.sort();
    r.sort();

    let mut result = 0;

    let mut i = 0;
    let mut j = 0;

    while i < l.len() && j < r.len() {
        if l[i] < r[j] {
            i += 1;
        }
        else if l[i] > r[j] {
            j += 1;
        }
        else {
            let elem = l[i];
            let mut l_count = 0;
            while i < l.len() && l[i] == elem {
                l_count += 1;
                i += 1;
            }
            let mut r_count = 0;
            while j < r.len() && r[j] == elem {
                r_count += 1;
                j += 1;
            }
            result += elem * l_count * r_count;
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
