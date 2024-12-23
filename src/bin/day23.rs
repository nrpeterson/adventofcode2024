use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use nom::character::complete::{alpha1, char, newline};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use nom::sequence::separated_pair;
use adventofcode2024::build_main;

struct Graph<'a> {
    verts: Vec<&'a str>,
    adjlist: HashMap<&'a str, HashSet<&'a str>>
}

impl<'a> Graph<'a> {
    fn from_edges(edges: Vec<(&'a str, &'a str)>) -> Graph {
        let mut adjlist = HashMap::new();
        for (a, b) in edges.into_iter() {
            adjlist.entry(a).or_insert_with(HashSet::new).insert(b);
            adjlist.entry(b).or_insert_with(HashSet::new).insert(a);
        }
        let mut verts: Vec<&str> = adjlist.keys().map(|&s| s).collect();
        verts.sort();
        Graph { verts, adjlist }
    }
}

fn parse_input(input: &str) -> IResult<&str, Graph> {
        map(
            separated_list1(
                newline,
                separated_pair(alpha1, char('-'), alpha1)
            ),
            |edges| Graph::from_edges(edges)
        )(input)
}

fn part1(input: &str) -> usize {
    let graph = parse_input(input).unwrap().1;

    let t_verts: Vec<&str> = graph.verts.iter()
        .filter(|k| k.starts_with('t'))
        .map(|&k| k)
        .collect();

    let mut triangles: HashSet<[&str; 3]> = HashSet::new();

    for a in t_verts.into_iter() {
        for (b, c) in graph.adjlist[&a].iter().tuple_combinations() {
            if graph.adjlist[b].contains(c) {
                let mut tri = [a, *b, *c];
                tri.sort();
                triangles.insert(tri);
            }
        }
    }
    triangles.len()
}

fn part2(input: &str) -> String {
    let graph = parse_input(input).unwrap().1;
    let mut best = Vec::new();
    let mut stack = Vec::new();

    graph.verts.iter().for_each(|&v| stack.push(vec![v]));

    while let Some(vs) = stack.pop() {
        let last = *vs.last().unwrap();

        let common_neighbors: Vec<&str> = graph.adjlist[last].iter()
            .filter(|&n| vs.iter().all(|v| graph.adjlist[v].contains(n)))
            .map(|&n| n)
            .collect();

        let choices: Vec<&str> = common_neighbors.into_iter()
            .filter(|&w| w > last)
            .collect();

        if vs.len() + choices.len() < best.len() {
            // No point -- most we could ever add won't beat our best known
            continue;
        }

        if choices.is_empty() {
            if vs.len() > best.len() {
                best = vs;
            }
            continue
        }

        for v in choices {
            let mut choice = vs.clone();
            choice.push(v);
            stack.push(choice);
        }
    }
    best.join(",")
}

build_main!("day23.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "co,de,ka,ta")
    }
}