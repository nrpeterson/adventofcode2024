use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use itertools::Itertools;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2024::build_main;

type Pos = (usize, usize);

fn parse_input(input: &str) -> Vec<Pos> {
    let result: IResult<&str, Vec<Pos>> = separated_list1(
        newline,
        separated_pair(
            map_res(digit1, str::parse::<usize>),
            char(','),
            map_res(digit1, str::parse::<usize>)
        )
    )(input);

    result.unwrap().1
}

#[derive(Eq, PartialEq)]
struct HeapElem { node: Pos, distance: usize }

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

struct Map {
    rows: usize,
    cols: usize,
    corrupted: HashSet<Pos>,
    best_path_nodes: Option<HashSet<Pos>>,
}

impl Map {
    fn new(rows: usize, cols: usize) -> Map {
        let mut best_path_nodes = HashSet::new();
        for i in 0..rows {
            best_path_nodes.insert((0, i));
        }
        for j in 0..cols {
            best_path_nodes.insert((rows - 1, j));
        }
        Map { rows, cols, corrupted: HashSet::new(), best_path_nodes: Some(best_path_nodes) }
    }

    fn corrupt(&mut self, pos: Pos) {
        self.corrupted.insert(pos);

        if let Some(nodes) = &self.best_path_nodes {
            if nodes.contains(&pos) {
                let start = (0, 0);
                let end = (self.rows - 1, self.cols - 1);
                self.best_path_nodes = self.best_path(start, end);
            }
        }
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut opts = Vec::new();
        let (i, j) = pos;
        if i > 0 { opts.push((i - 1, j)); }
        if i < self.rows - 1 { opts.push((i + 1, j)); }
        if j > 0 { opts.push((i, j - 1)); }
        if j < self.cols - 1 { opts.push((i, j + 1)); }

        opts.iter().filter(|&x| !self.corrupted.contains(x)).cloned().collect()
    }

    fn best_path(&self, from: Pos, to: Pos) -> Option<HashSet<Pos>> {
        let mut result: HashMap<Pos, (usize, Option<Pos>)> =
            (0..self.rows).cartesian_product(0..self.cols)
                .filter(|pos| !self.corrupted.contains(pos))
                .map(|pos| (pos, if pos == from { 0 } else { usize::MAX }))
                .map(|(pos, dist)| (pos, (dist, None)))
                .collect();

        let mut heap = BinaryHeap::new();
        heap.push(HeapElem { node: from, distance: 0 });

        while let Some(HeapElem { node, distance }) = heap.pop() {
            if node == to {
                break;
            }

            if result[&node].0 < distance { continue; }

            self.neighbors(node).iter().for_each(|&n| {
                let (cur_dist, cur_pred) = result.get_mut(&n).unwrap();
                if *cur_dist > distance + 1 {
                    *cur_dist = distance + 1;
                    *cur_pred = Some(node);
                    heap.push(HeapElem { node: n, distance: distance + 1 });
                }
            })
        }

        let distance = result[&to].0;

        if distance == usize::MAX {
            None
        } else {

            let mut path_nodes = HashSet::new();
            path_nodes.insert(to);

            let mut cur = to;
            while let Some(n) = result[&cur].1 {
                path_nodes.insert(n);
                cur = n;
            }

            Some(path_nodes)
        }
    }
}

fn part1(input: &str) -> usize {
    let mut map = Map::new(71, 71);
    let corrupted = parse_input(input);
    corrupted[..1024].iter().for_each(|&pos| map.corrupt(pos));
    map.best_path_nodes.expect("There should be a path").len()
}

fn part2(input: &str) -> String {
    let mut map = Map::new(71, 71);
    let mut corrupted = parse_input(input).into_iter();

    while let Some(pos) = corrupted.next() {
        map.corrupt(pos);
        if map.best_path_nodes.is_none() { return format!("{},{}", pos.0, pos.1) }
    }

    panic!("We didn't ever block the path!")
}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);