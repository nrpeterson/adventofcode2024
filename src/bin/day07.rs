use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2024::build_main;

struct Problem {
    expected: u64,
    nums: Vec<u64>
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn problem(input: &str) -> IResult<&str, Problem> {
    map(
        separated_pair(number, tag(": "), separated_list1(space1, number)),
        |(expected, nums)| Problem { expected, nums },
    )(input)
}

fn parse_input(input: &str) -> Vec<Problem> {
    let parsed: IResult<&str, Vec<Problem>> = separated_list1(newline, problem)(input);
    parsed.expect("parsing error").1
}

fn num_solutions<F>(problem: &Problem, f: F) -> usize
    where F: Fn(u64, u64) -> Vec<Option<u64>> {
    let starts = problem.nums[1..].iter()
        .rfold(vec![problem.expected], |acc, &x| {
            acc.into_iter()
                .flat_map(|y| f(y, x).into_iter().flatten())
                .collect()
        });

    starts.into_iter().filter(|&x| x == problem.nums[0]).count()
}

fn try_sub(result: u64, addend: u64) -> Option<u64> {
    if result <= addend { None } else { Some(result - addend) }
}

fn try_div(result: u64, divisor: u64) -> Option<u64> {
    if result % divisor == 0 { Some(result / divisor) } else { None }
}

fn part1(input: &str) -> u64 {
    fn ops(y: u64, x: u64) -> Vec<Option<u64>> {
        vec![try_sub(y, x), try_div(y, x)]
    }
    parse_input(input).into_iter()
        .filter(|p| num_solutions(p, ops) > 0)
        .map(|p| p.expected)
        .sum()
}

fn try_split(joined: u64, second: u64) -> Option<u64> {
    if second == 0 {
        if joined % 10 == 0 { Some(joined / 10) } else { None }
    }
    else if second >= joined {
        None
    }
    else {
        let log = (second as f64).log10().floor() as u32 + 1;
        let mask = 10u64.pow(log);
        let rem = joined - second;
        if rem % mask == 0 { Some(rem / mask) } else { None }
    }
}

fn part2(input: &str) -> u64 {
    fn ops(y: u64, x: u64) -> Vec<Option<u64>> {
        vec![try_sub(y, x), try_div(y, x), try_split(y, x)]
    }
    parse_input(input).into_iter()
        .filter(|p| num_solutions(p, ops) > 0)
        .map(|p| p.expected)
        .sum()
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }
}