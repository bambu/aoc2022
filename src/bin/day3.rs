use std::collections::HashSet;

fn main() {
    let rucksacks: Vec<Rucksack> = include_str!("../../inputs/input3.txt")
        .lines()
        .map(Rucksack::new)
        .collect();

    println!("Day3 Part1: {}", part1(&rucksacks));
    println!("Day3 Part2: {}", part2(&rucksacks));
}

fn part1(sacks: &[Rucksack]) -> usize {
    sacks
        .iter()
        .map(|s| s.in_both().unwrap())
        .map(|c| match c {
            'a'..='z' => ('a'..='z').position(|x| x == c).unwrap() + 1,
            'A'..='Z' => ('A'..='Z').position(|x| x == c).unwrap() + 27,
            _ => 0,
        })
        .sum::<usize>()
}

fn part2(sacks: &[Rucksack]) -> usize {
    let mut result = 0;
    for chunk in sacks.chunks(3) {
        let sets = chunk
            .iter()
            .map(|x| x.full_set())
            .collect::<Vec<HashSet<char>>>();
        let value = sets[0]
            .iter()
            .filter(|k| sets[1].contains(k) && sets[2].contains(k))
            .next()
            .unwrap();
        result += match value {
            'a'..='z' => ('a'..='z').position(|x| x == *value).unwrap() + 1,
            'A'..='Z' => ('A'..='Z').position(|x| x == *value).unwrap() + 27,
            _ => 0,
        };
    }
    result
}

#[derive(Debug)]
pub struct Rucksack {
    pub compartment_1: String,
    pub compartment_2: String,
}

impl Rucksack {
    pub fn new(contents: &str) -> Self {
        let (comp1, comp2) = contents.split_at(contents.len() / 2);
        Self {
            compartment_1: comp1.to_owned(),
            compartment_2: comp2.to_owned(),
        }
    }

    pub fn in_both(&self) -> Option<char> {
        let comp1: HashSet<char> = self.compartment_1.chars().collect();
        let comp2: HashSet<char> = self.compartment_2.chars().collect();

        comp1
            .intersection(&comp2)
            .next()
            .and_then(|x| Some(x.to_owned()))
    }

    pub fn full_set(&self) -> HashSet<char> {
        let comp1: HashSet<char> = self.compartment_1.chars().collect();

        comp1
            .union(&self.compartment_2.chars().collect())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let rucksacks: Vec<Rucksack> = INPUT.lines().map(Rucksack::new).collect();
        assert_eq!(rucksacks[0].in_both(), Some('p'));
        assert_eq!(rucksacks[1].in_both(), Some('L'));
        assert_eq!(rucksacks[2].in_both(), Some('P'));
        assert_eq!(rucksacks[3].in_both(), Some('v'));
        assert_eq!(rucksacks[4].in_both(), Some('t'));
        assert_eq!(rucksacks[5].in_both(), Some('s'));
        assert_eq!(part1(&rucksacks), 157);
    }

    #[test]
    fn test_part2() {
        let rucksacks: Vec<Rucksack> = INPUT.lines().map(Rucksack::new).collect();
        assert_eq!(part2(&rucksacks), 70)
    }
}
