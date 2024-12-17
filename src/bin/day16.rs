use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use adventofcode2024::build_main;

#[derive(Copy, Clone, Eq, PartialEq)]
enum CellType { Start, End, Empty, Wall }
use CellType::*;


#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
enum Direction { Up, Down, Left, Right }
use Direction::*;


struct Maze {
    cells: Vec<Vec<CellType>>,
    rows: usize,
    cols: usize,
    start: (usize, usize),
    end: (usize, usize)
}

impl Maze {
    fn from_cells(cells: Vec<Vec<CellType>>) -> Maze {
        let rows = cells.len();
        let cols = cells[0].len();
        let start = (0..rows).cartesian_product(0..cols)
            .find(|&(i, j)| cells[i][j] == Start)
            .unwrap();

        let end = (0..cols).cartesian_product(0..rows)
            .find(|&(i, j)| cells[i][j] == End)
            .unwrap();

        Maze { cells, rows, cols, start, end }
    }

    fn next_pos(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        match (direction, pos) {
            (Up, (i, _)) if i == 0 => None,
            (Up, (i, j)) => Some((i - 1, j)),
            (Down, (i, _)) if i == self.rows - 1 => None,
            (Down, (i, j)) => Some((i + 1, j)),
            (Left, (_, j)) if j == 0 => None,
            (Left, (i, j)) => Some((i, j - 1)),
            (Right, (_, j)) if j == self.cols - 1 => None,
            (Right, (i, j)) => Some((i, j + 1))
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Maze> {
    map(
        separated_list1(
            newline,
            many1(
                alt((
                    value(Start, char('S')),
                    value(End, char('E')),
                    value(Empty, char('.')),
                    value(Wall, char('#'))
                ))
            )
        ), |cells| Maze::from_cells(cells)
    )(input)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
struct Node { x: usize, y: usize, direction: Direction }

struct Graph {
    adj_list: HashMap<Node, Vec<(Node, usize)>>
}

impl Graph {
    fn from_maze(maze: &Maze) -> Graph {
        let mut adj_list = HashMap::new();
        for (x, y) in (0..maze.rows).cartesian_product(0..maze.cols) {
            let cell_type = maze.cells[x][y];
            if cell_type == Wall {
                continue;
            }

            for &direction in [Up, Down, Left, Right].iter() {
                let node = Node { x, y, direction };
                let neighbors = adj_list.entry(node).or_insert_with(Vec::new);

                // Can either move to next space in direction (without turning), or turn.
                if let Some((i, j)) = maze.next_pos((x, y), direction) {
                    if maze.cells[i][j] != Wall {
                        neighbors.push((Node { x: i, y: j, direction }, 1));
                    }
                }

                let turns = match direction {
                    Up => [Left, Right],
                    Down => [Left, Right],
                    Right => [Up, Down],
                    Left => [Up, Down]
                };

                for new_dir in turns {
                    neighbors.push((Node { x, y, direction: new_dir }, 1000));
                }
            }
        }

        Graph { adj_list }
    }
}

#[derive(Eq, PartialEq)]
struct HeapElem { node: Node, cost: usize }

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn dijkstra(graph: &Graph, from: Node) -> HashMap<Node, (usize, HashSet<Node>)> {
    let mut result: HashMap<Node, (usize, HashSet<Node>)> =
        graph.adj_list.keys().map(|&n| (n, (usize::MAX, HashSet::new()))).collect();

    result.get_mut(&from).unwrap().0 = 0;

    let mut heap = BinaryHeap::new();
    heap.push(HeapElem { node: from, cost: 0 });

    while let Some(HeapElem { node, cost }) = heap.pop() {
        if result[&node].0 > cost { continue; }

        for &(neighbor, weight) in graph.adj_list[&node].iter() {
            if let Some((cur_dist, cur_preds)) = result.get_mut(&neighbor) {
                if *cur_dist == cost + weight {
                    cur_preds.insert(node);
                }
                else if *cur_dist > cost + weight {
                    cur_preds.clear();
                    cur_preds.insert(node);
                    *cur_dist = cost + weight;
                    heap.push(HeapElem { node: neighbor, cost: cost + weight });
                }
            }
        }
    }

    result
}

fn part1(input: &str) -> usize {
    let maze = parse_input(input).unwrap().1;
    let graph = Graph::from_maze(&maze);
    let start = Node {x: maze.start.0, y: maze.start.1, direction: Right };
    let result = dijkstra(&graph, start);

    [Up, Down, Left, Right].iter()
        .map(|&d| Node { x: maze.end.0, y: maze.end.1, direction: d })
        .map(|n| result[&n].0)
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let maze = parse_input(input).unwrap().1;
    let graph = Graph::from_maze(&maze);
    let start = Node {x: maze.start.0, y: maze.start.1, direction: Right };

    let result = dijkstra(&graph, start);

    let end = [Up, Down, Left, Right].iter()
        .map(|&d| Node { x: maze.end.0, y: maze.end.1, direction: d })
        .min_by_key(|n| result[n].0)
        .unwrap();

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(end);

    while let Some(node) = queue.pop_front() {
        seen.insert((node.x, node.y));
        result[&node].1.iter().for_each(|&n| queue.push_back(n));
    }

    seen.len()
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 7036);
        assert_eq!(part1(TEST_INPUT_2), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_1), 45);
        assert_eq!(part2(TEST_INPUT_2), 64);
    }
}