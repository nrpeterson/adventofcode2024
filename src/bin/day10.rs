use std::collections::{HashMap, HashSet, VecDeque};
use adventofcode2024::build_main;

struct Digraph {
    adj_list: HashMap<(usize, usize), Vec<(usize, usize)>>,
    zeroes: Vec<(usize, usize)>,
    nines: HashSet<(usize, usize)>
}

impl Digraph {
    fn count_trails_from(&self, node: (usize, usize)) -> HashMap<(usize, usize), usize> {
        let mut result = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while let Some(v) = queue.pop_front() {
            if self.nines.contains(&v) {
                *(result.entry(v).or_insert(0)) += 1;
            }
            for &u in self.adj_list.get(&v).unwrap_or(&vec![]) {
                queue.push_back(u);
            }
        }

        result
    }
}

fn parse_input(input: &str) -> Digraph {
    let topo: Vec<Vec<usize>> = input.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        }).collect();

    let mut zeroes = Vec::new();
    let mut nines = HashSet::new();

    let num_rows = topo.len();
    let num_cols = topo[0].len();
    let mut adj_list: HashMap<(usize, usize), Vec<(usize, usize)>>= HashMap::new();

    for i in 0..num_rows {
        for j in 0..num_cols {
            let val = topo[i][j];

            if val == 0 {
                zeroes.push((i, j));
            }

            if val == 9 {
                nines.insert((i, j));
            }

            let mut neighbors = Vec::new();
            if i > 0 {
                neighbors.push((i - 1, j));
            }
            if i < num_rows - 1 {
                neighbors.push((i + 1, j));
            }
            if j > 0 {
                neighbors.push((i, j - 1));
            }
            if j < num_cols - 1 {
                neighbors.push((i, j + 1));
            }

            neighbors.iter().filter(|&&(x, y)| topo[x][y] == val + 1)
                .for_each(|&(x, y)| {
                    adj_list.entry((i, j)).or_default().push((x, y));
                })
        }
    }

    Digraph { adj_list, zeroes, nines }
}

fn part1(input: &str) -> usize {
    let digraph = parse_input(input);

    digraph.zeroes.iter()
        .map(|&v| {
            digraph.count_trails_from(v).values()
                .filter(|&&u| u > 0)
                .count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let digraph = parse_input(input);

    digraph.zeroes.iter()
        .map(|&v| {
            digraph.count_trails_from(v).values().sum::<usize>()
        }).sum()
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
    }
}