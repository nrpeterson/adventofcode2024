use adventofcode2024::build_main;
use std::cmp::min;

#[derive(Copy, Clone, Debug)]
struct Segment { file_id: Option<usize>, size: usize, index: usize }

#[derive(Debug)]
struct Disk {
    segments: Vec<Segment>
}

const TRIANGULAR: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

impl Disk {
    fn read(input: &str) -> Disk {
        let segments: Vec<Segment> = input.trim().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
            .fold((Vec::new(), 0), |(mut acc, index), (i, size)| {
                let file_id = if i % 2 == 0 { Some(i / 2) } else { None };
                acc.push(Segment { file_id, size, index });
                (acc, index + size)
            }).0;

        Disk { segments }
    }
}


fn part1(input: &str) -> usize {
    let mut disk = Disk::read(input);

    let mut total = 0;
    let mut i = 0;
    let mut j = disk.segments.len() - 1;

    while i < j {
        let (seg1, seg2) = (disk.segments[i], disk.segments[j]);
        match (seg1, seg2) {
            (Segment { size, ..}, _) if size == 0 => { i += 1; },
            (_, Segment { size, ..}) if size == 0 => { j -= 1; },
            (_, Segment { file_id: None, .. }) => { j -= 1; },
            (Segment { file_id: Some(file_id), size, index}, _) => {
                total += file_id * (size * index + TRIANGULAR[size]);
                i += 1;
            },
            (
                Segment { file_id: None, size: gap_size, index: gap_index},
                Segment { file_id: Some(file_id), size: file_size, ..}
            ) => {
                let size = min(file_size, gap_size);
                disk.segments[i].size -= size;
                disk.segments[i].index += size;
                disk.segments[j].size -= size;
                total += file_id * (gap_index * size + TRIANGULAR[size]);
            }
        }
    }

    if let Segment{ file_id: Some(file_id), size, index } = disk.segments[i] {
        total += file_id * (index * size + TRIANGULAR[size]);
    }

    total
}

#[derive(Debug, Copy, Clone)]
struct File { file_id: usize, size: usize, index: usize }

#[derive(Debug, Copy, Clone)]
struct Gap { size: usize, index: usize }

fn part2(input: &str) -> usize {
    let disk = Disk::read(input);

    let files: Vec<File> = disk.segments.iter().filter_map(|&seg| {
        seg.file_id.map(|file_id| File { file_id, size: seg.size, index: seg.index })
    }).collect();

    let mut gaps: Vec<Gap> = disk.segments.iter()
        .filter(|seg| seg.file_id.is_none())
        .map(|seg| Gap { size: seg.size, index: seg.index })
        .collect();

    let mut total = 0;

    for &file in files.iter().rev() {
        if let Some((i, gap)) = gaps.iter().enumerate()
            .filter(|&(_, &gap)| gap.index < file.index)
            .find(|&(_, &gap)| gap.size >= file.size) {
            total += file.file_id * (file.size * gap.index + TRIANGULAR[file.size]);
            gaps[i].size -= file.size;
            gaps[i].index += file.size;
        }
        else {
            total += file.file_id * (file.size * file.index + TRIANGULAR[file.size])
        }
    }

    total
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}