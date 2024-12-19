use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use adventofcode2024::build_main;

struct Input {
    components: Vec<String>,
    targets: Vec<String>
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        separated_pair(
            separated_list1(tag(", "), map(alpha1, |s: &str| s.to_owned())),
            many1(newline),
            separated_list1(newline, map(alpha1, |s: &str| s.to_owned()))
        ),
        |(components, targets)| Input { components, targets }
    )(input)
}

fn ways_to_build(target: &String, from: &Vec<String>) -> usize {
    let mut counts = vec![None; target.len() + 1];
    counts[0] = Some(1);
    let mut stack = Vec::new();
    stack.push(target.len());

    while let Some(&n) = stack.last() {
        if counts[n].is_some() {
            stack.pop();
            continue
        }

        let suffixes: Vec<&String> = from.iter()
            .filter(|&s| target[..n].ends_with(s))
            .collect();

        let unsat: Vec<&String> = suffixes.iter()
            .filter(|s| counts[n - s.len()].is_none())
            .map(|&s| s)
            .collect();

        if unsat.is_empty() {
            let result = suffixes.iter()
                .map(|s| counts[n - s.len()].unwrap())
                .sum();

            counts[n] = Some(result);
            stack.pop();
        }
        else {
            unsat.iter().for_each(|&s| stack.push(n - s.len()));
        }
    }

    counts[target.len()].unwrap()
}

fn part1(input: &str) -> usize {
    let Input { components, targets } = parse_input(input).expect("Input is valid").1;

    targets.iter()
        .map(|target| ways_to_build(target, &components))
        .filter(|&n| n > 0)
        .count()
}

fn part2(input: &str) -> usize {
    let Input { components, targets } = parse_input(input).expect("Input is valid").1;

    targets.iter()
        .map(|target| ways_to_build(target, &components))
        .sum()
}

build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 16);
    }
}