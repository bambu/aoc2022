use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let assignments: Vec<AssignmentGroup> = include_str!("../../inputs/input4.txt")
        .lines()
        .map(AssignmentGroup::new)
        .collect();

    println!("Day4 Part1: {}", part1(&assignments));
    println!("Day4 Part2: {}", part2(&assignments));
}

fn part1(assignments: &[AssignmentGroup]) -> u32 {
    assignments
        .iter()
        .map(|x| u32::from(x.are_overlapping()))
        .sum()
}

fn part2(assignments: &[AssignmentGroup]) -> u32 {
    assignments.iter().map(|x| u32::from(x.any_overlap())).sum()
}

#[derive(Debug)]
struct AssignmentGroup {
    elf1: RangeInclusive<u32>,
    elf2: RangeInclusive<u32>,
}

impl AssignmentGroup {
    fn new(data: &str) -> Self {
        let mut iter = data.split(&['-', ',']).map(|x| x.parse::<u32>().unwrap());

        Self {
            elf1: iter.next().unwrap()..=iter.next().unwrap(),
            elf2: iter.next().unwrap()..=iter.next().unwrap(),
        }
    }

    fn are_overlapping(&self) -> bool {
        if self.elf1.contains(self.elf2.start()) && self.elf1.contains(self.elf2.end()) {
            return true;
        }
        if self.elf2.contains(self.elf1.start()) && self.elf2.contains(self.elf1.end()) {
            return true;
        }

        false
    }

    fn any_overlap(&self) -> bool {
        let num_intersecting = self
            .elf1
            .clone()
            .collect::<HashSet<u32>>()
            .intersection(&self.elf2.clone().collect::<HashSet<u32>>())
            .count();

        num_intersecting > 0
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let assignments: Vec<AssignmentGroup> = INPUT.lines().map(AssignmentGroup::new).collect();
        assert_eq!(part1(&assignments), 2);
    }

    #[test]
    fn test_part2() {
        let assignments: Vec<AssignmentGroup> = INPUT.lines().map(AssignmentGroup::new).collect();
        assert_eq!(part2(&assignments), 4);
    }
}
