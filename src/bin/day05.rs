use adventofcode2024::build_main;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
struct Rule(usize, usize);

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(separated_pair(number, char('|'), number), |(a, b)| Rule(a, b))(input)
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<usize>>) {
    separated_pair(
        separated_list1(newline, rule),
        pair(newline, newline),
        separated_list1(newline, separated_list1(char(','), number))
    )(input).unwrap().1
}

fn is_top_sorted(succs: &HashMap<usize, Vec<usize>>, pages: &Vec<usize>) -> bool {
    let mut seen = HashSet::new();
    for page in pages {
        if let Some(ss) = succs.get(&page) {
            if ss.iter().any(|&s| seen.contains(&s)) {
                return false;
            }
        }
        seen.insert(page);
    }

    true
}

fn part1(input: &str) -> usize {
    let (rules, page_groups) = parse_input(input);

    let succs: HashMap<usize, Vec<usize>> = rules.iter()
        .fold(HashMap::new(), |mut acc, &Rule(a, b)| {
            acc.entry(a).or_default().push(b);
            acc
        });

    page_groups.iter().filter(|&pages| is_top_sorted(&succs, pages))
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum()
}

fn top_sorted(pages: &Vec<usize>, rules: &Vec<Rule>) -> Option<Vec<usize>> {
    let page_set: HashSet<usize> = pages.iter().cloned().collect();

    let mut preds: HashMap<usize, HashSet<usize>> = rules.iter()
        .filter(|&Rule(u, v)| page_set.contains(u) && page_set.contains(v))
        .fold(HashMap::new(), |mut acc, &Rule(u, v)| {
            acc.entry(v).or_insert_with(HashSet::new).insert(u);
            acc
        });

    let mut no_preds: Vec<usize> = page_set.into_iter()
        .filter(|&page| preds.entry(page).or_default().is_empty())
        .collect();

    let mut result = Vec::new();

    while let Some(x) = no_preds.pop() {
        result.push(x);
        for (&k, vs) in preds.iter_mut() {
            if vs.contains(&x) {
                vs.remove(&x);
                if vs.is_empty() {
                    no_preds.push(k);
                }
            }
        }
    }

    if preds.iter().any(|(_, vs)| !vs.is_empty()) {
        None
    }
    else {
        Some(result)
    }
}

fn part2(input: &str) -> usize {
    let (rules, page_groups) = parse_input(input);

    let succs: HashMap<usize, Vec<usize>> = rules.iter()
        .fold(HashMap::new(), |mut acc, &Rule(a, b)| {
            acc.entry(a).or_default().push(b);
            acc
        });

    page_groups.iter()
        .filter(|&pages| !is_top_sorted(&succs, pages))
        .map(|pages| top_sorted(pages, &rules).unwrap())
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum()
}

build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }
}