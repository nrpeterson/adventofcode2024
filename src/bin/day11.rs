use std::collections::HashMap;
use itertools::Itertools;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use adventofcode2024::build_main;

fn parse_input(input: &str) -> Vec<usize> {
    let number = map_res(digit1, |s: &str| s.parse());
    let result: IResult<&str, Vec<usize>> = separated_list1(space1, number)(input);
    result.unwrap().1
}

fn num_digits(n: usize) -> usize {
    n.ilog10() as usize + 1
}

fn stones_after_blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else {
        let d = num_digits(stone);
        if d % 2 == 0 {
            let mask = 10usize.pow(d as u32 / 2);
            vec![stone / mask, stone % mask]
        } else {
            vec![2024 * stone]
        }
    }
}

fn count_after_blinks(num_blinks: usize, stones: Vec<usize>) -> usize {
    let mut stone_counts = stones.into_iter().counts();

    for _ in 0..num_blinks {
        let mut new_counts = HashMap::new();
        for (num, count) in stone_counts.into_iter() {
            let new_stones = stones_after_blink(num);
            new_stones.into_iter().for_each(|n| {
                *new_counts.entry(n).or_insert(0) += count
            });
        }
        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}

fn part1(input: &str) -> usize {
    let stones = parse_input(input);
    count_after_blinks(25, stones)
}

fn part2(input: &str) -> usize {
    let stones = parse_input(input);
    count_after_blinks(75, stones)
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::part1;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
    }
}