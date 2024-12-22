use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2024::build_main;

fn numpad_pos(key: char) -> (usize, usize) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!("Invalid numpad key")
    }
}

fn numpad_paths(from: char, to: char) -> Vec<String> {
    let (i0, j0) = numpad_pos(from);
    let (i1, j1) = numpad_pos(to);

    let vert_char = if i0 < i1 { 'v' } else { '^' };
    let horiz_char = if j0 < j1 { '>' } else { '<' };

    let vert = (0..i0.abs_diff(i1)).map(|_| vert_char).collect::<String>();
    let horiz = (0..j0.abs_diff(j1)).map(|_| horiz_char).collect::<String>();

    if i0 == 3 && j1 == 0 {
        vec![format!("{vert}{horiz}A")]
    }
    else if i1 == 3 && j0 == 0 {
        vec![format!("{horiz}{vert}A")]
    }
    else if vert.is_empty() || horiz.is_empty() {
        vec![format!("{horiz}{vert}A")]
    }
    else {
        let vh = format!("{vert}{horiz}A");
        let hv = format!("{horiz}{vert}A");
        vec![vh, hv]
    }
}

fn dirpad_paths(from: char, to: char) -> Vec<String> {
    let result = match (from, to) {
        ('A', '^') => vec!["<A"],
        ('A', '>') => vec!["vA"],
        ('A', 'v') => vec!["<vA", "v<A"],
        ('A', '<') => vec!["v<<A"],
        ('^', 'A') => vec![">A"],
        ('^', '>') => vec![">vA", "v>A"],
        ('^', 'v') => vec!["vA"],
        ('^', '<') => vec!["v<A"],
        ('>', 'A') => vec!["^A"],
        ('>', '^') => vec!["<^A", "^<A"],
        ('>', 'v') => vec!["<A"],
        ('>', '<') => vec!["<<A"],
        ('v', 'A') => vec!["^>A", ">^A"],
        ('v', '^') => vec!["^A"],
        ('v', '>') => vec![">A"],
        ('v', '<') => vec!["<A"],
        ('<', 'A') => vec![">>^A"],
        ('<', '^') => vec![">^A"],
        ('<', '>') => vec![">>A"],
        ('<', 'v') => vec![">A"],
        _ => vec!["A"]
    };

    result.iter().map(|&s| s.to_owned()).collect()
}

struct Cache {
    lookup: HashMap<(String, usize), usize>
}

impl Cache {
    fn new() -> Cache {
        Cache { lookup: HashMap::new() }
    }
    fn dirpad_cost_for_seq(&mut self, seq: &String, intermediate_robots: usize) -> usize {
        if intermediate_robots == 0 {
            return seq.len()
        }

        let key = (seq.clone(), intermediate_robots);
        if self.lookup.contains_key(&key) {
            self.lookup[&key].clone()
        } else {
            let mut s = "A".to_owned();
            s.push_str(seq);

            let mut result = 0;

            for (from, to) in s.chars().tuple_windows() {
                let paths = dirpad_paths(from, to);
                let min_cost = if intermediate_robots == 0 {
                    paths.iter()
                        .map(|s| s.len())
                        .min()
                        .expect("No paths found")
                } else {
                    paths.iter()
                        .map(|path| self.dirpad_cost_for_seq(path, intermediate_robots - 1))
                        .min()
                        .expect("No paths found")
                };
                result += min_cost;
            }

            self.lookup.insert(key, result);
            result
        }
    }

    fn numpad_cost_for_seq(&mut self, seq: &str, intermediate_robots: usize) -> usize {
        let mut result = 0;
        let mut s = "A".to_owned();
        s.push_str(seq);

        for (from, to) in s.chars().tuple_windows() {
            let best_cost = numpad_paths(from, to).into_iter()
                .map(|path| {
                    if intermediate_robots > 0 {
                        self.dirpad_cost_for_seq(&path, intermediate_robots)
                    }
                    else {
                        path.len()
                    }
                })
                .min()
                .unwrap();

            result += best_cost;
        }

        result
    }
}

fn numeric_part(seq: &str) -> usize {
    seq.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .fold(0, |cur, next| 10 * cur + next)
}

fn part1(input: &str) -> usize {
    let mut cache = Cache::new();

    input.lines()
        .map(|seq| numeric_part(seq) * cache.numpad_cost_for_seq(seq, 2))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cache = Cache::new();

    input.lines()
        .map(|seq| numeric_part(seq) * cache.numpad_cost_for_seq(seq, 25))
        .sum()
}

build_main!("day21.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 126384);
    }
}