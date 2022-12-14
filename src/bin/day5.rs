use std::convert::Into;

fn main() {
    let (ship, action) = process_input(include_str!("../../inputs/input5.txt"));
    println!("Day5 Part1: {}", part1(&ship, &action));
    println!("Day5 Part2: {}", part2(&ship, &action));
}

fn part1(ship: &Ship, actions: &[Action]) -> String {
    let mut ship = ship.clone();
    actions.iter().for_each(|x| match x {
        Action::Move(count, from, to) => ship.move_many(*count, *from, *to),
    });

    ship.stacks
        .iter()
        .filter_map(|s| if s.is_empty() { None } else { s.chars().last() })
        .collect::<String>()
}

fn part2(ship: &Ship, actions: &[Action]) -> String {
    let mut ship = ship.clone();
    actions.iter().for_each(|x| match x {
        Action::Move(count, from, to) => ship.move_group(*count, *from, *to),
    });

    ship.stacks
        .iter()
        .filter_map(|s| if s.is_empty() { None } else { s.chars().last() })
        .collect::<String>()
}

#[derive(Debug, Clone)]
struct Ship {
    stacks: Vec<String>,
}

impl Ship {
    fn new(map_str: &[&str]) -> Self {
        let mut map_str = map_str.to_owned();
        let mut map: Vec<String> = Vec::new();

        let last_line = map_str.pop().unwrap();
        for (x, c) in last_line.chars().enumerate() {
            let mut stack: String = String::new();
            if c.is_numeric() {
                for y in (0..map_str.len()).rev() {
                    let id = map_str[y].chars().nth(x).unwrap();
                    if id.is_alphabetic() {
                        stack.push(id);
                    }
                }
                map.push(stack);
            }
        }

        Self { stacks: map }
    }

    fn move_one(&mut self, from: usize, to: usize) {
        let c = self.stacks[from - 1].pop().unwrap();
        self.stacks[to - 1].push(c);
    }

    fn move_many(&mut self, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            self.move_one(from, to);
        }
    }

    fn move_group(&mut self, count: usize, from: usize, to: usize) {
        let from_start = self.stacks[from-1].len() - count;
        let crates = self.stacks[from-1].drain(from_start..).collect::<Vec<char>>();
        self.stacks[to-1].extend(crates);
    }
}

#[derive(Debug)]
enum Action {
    Move(usize, usize, usize),
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        let cmd = s.split(' ').collect::<Vec<&str>>();
        match cmd.first() {
            Some(&"move") => Self::Move(
                cmd[1].parse::<usize>().unwrap(),
                cmd[3].parse::<usize>().unwrap(),
                cmd[5].parse::<usize>().unwrap(),
            ),
            _ => panic!("Unknown command!"),
        }
    }
}

fn process_input(data: &str) -> (Ship, Vec<Action>) {
    let mut lines = data.lines();
    let map_str = lines
        .by_ref()
        .take_while(|&x| !x.is_empty())
        .collect::<Vec<&str>>();
    let actions = lines.map(Into::into).collect::<Vec<Action>>();

    (Ship::new(&map_str), actions)
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let (ship, action) = process_input(INPUT);
        assert_eq!(part1(&ship, &action), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (ship, action) = process_input(INPUT);
        assert_eq!(part2(&ship, &action), "MCD");
    }
}
