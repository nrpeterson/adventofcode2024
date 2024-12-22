use itertools::Itertools;
use adventofcode2024::build_main;

struct SecretNumber {
    num: usize
}

impl SecretNumber {
    fn new(num: usize) -> Self {
        SecretNumber { num }
    }
}

impl Iterator for SecretNumber {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new = self.num;
        new = ((new << 6) ^ new) & 16777215;
        new = ((new >> 5) ^ new) & 16777215;
        new = ((new << 11) ^ new) & 16777215;

        let result = Some(self.num);
        self.num = new;

        result
    }
}

type FourDiffs = (isize, isize, isize, isize);

fn to_index(f: FourDiffs) -> usize {
    let (a, b, c, d) = f;
    [a, b, c, d].map(|x| (x + 9) as usize)
        .iter().fold(0, |acc, &next| 19*acc + next)
}

fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|n| {
            let mut s = SecretNumber::new(n);
            s.nth(2000).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut bananas = vec![0; 130321];

    for line in input.lines() {
        let mut seen = vec![false; 130321];
        let s = SecretNumber::new(line.parse::<usize>().unwrap());

        s.take(2001)
            .map(|n| n % 10)
            .tuple_windows().map(|(a, b)| (b, (b as isize) - (a as isize)))
            .tuple_windows().for_each(|((_, d0), (_, d1), (_, d2), (n, d3))| {
                let i = to_index((d0, d1, d2, d3));
                if !seen[i] {
                    seen[i] = true;
                    bananas[i] += n;
                }
            });
    }

    bananas.into_iter().max().unwrap()
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generation() {
        let first_10: Vec<usize> = SecretNumber::new(123)
            .dropping(1)
            .take(10)
            .collect();

        let expected = vec![
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254
        ];
        assert_eq!(first_10, expected);
    }

    #[test]
    fn test_part1() {
        let input = "1\n10\n100\n2024";
        assert_eq!(part1(input), 37327623);
    }

    #[test]
    fn test_part2() {
        let input = "1\n2\n3\n2024";
        assert_eq!(part2(input), 23);
    }
}