use std::convert::Into;

fn main() {
    let input = include_str!("../../inputs/input2.txt");

    let rounds: Vec<Vec<Move>> = input.clone().lines().map(|line| {
        line.split(' ').map(Into::into).collect()
    }).collect();

    println!("Day2 Part1: {}", part1(&rounds));

    let rounds2: Vec<(Move, Outcome)> = input.lines().map(|line| {
        let mut iter = line.splitn(2, ' ');
        (iter.next().unwrap().into(), iter.next().unwrap().into())
    }).collect();
    println!("Day2 Part2: {}", part2(&rounds2));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => {panic!("Unsupported outcome");}
        }
    }
}

impl From<Outcome> for i32 {
    fn from(m: Outcome) -> Self {
        match m {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => {panic!("Unsupported move");}
        }
    }
}

impl From<Move> for i32 {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}

impl Move {
    fn play(self, player2: Move) -> i32 {
        let mut result: i32 = self.into();
        result += if self == player2 {
            3
        } else {
            match (self, player2) {
                (Move::Rock, Move::Scissor) => 6,
                (Move::Rock, Move::Paper) => 0,
                (Move::Scissor, Move::Paper) => 6,
                (Move::Scissor, Move::Rock) => 0,
                (Move::Paper, Move::Rock) => 6,
                (Move::Paper, Move::Scissor) => 0,
                _ => { panic!("Unreachable!");}
            }
        };

        result
    }

    fn guess(self, outcome: Outcome) -> i32 {
        let outcome_val: i32 = outcome.into();
        let result: i32 = if outcome == Outcome::Draw {
            self.into()
        } else {
            match (self, outcome) {
                (Move::Rock, Outcome::Lose) => Move::Scissor.into(),
                (Move::Rock, Outcome::Win) => Move::Paper.into(),
                (Move::Scissor, Outcome::Lose) => Move::Paper.into(),
                (Move::Scissor, Outcome::Win) => Move::Rock.into(),
                (Move::Paper, Outcome::Lose) => Move::Rock.into(),
                (Move::Paper, Outcome::Win) => Move::Scissor.into(),
                _ => { panic!("Unreachable!");}
            }
        };

        result + outcome_val
    }
}

fn part1(rounds: &[Vec<Move>]) -> i32 {
    rounds.iter().map(|x| x[1].play(x[0])).sum()
}

fn part2(rounds: &[(Move, Outcome)]) -> i32 {
    rounds.iter().map(|x| x.0.guess(x.1)).sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "A Y
B X
C Z";
    
    #[test]
    fn test_part1() {
        let rounds: Vec<Vec<Move>> = INPUT.lines().map(|line| {
            line.split(' ').map(Into::into).collect()
        }).collect();

        assert_eq!(rounds[0][1].play(rounds[0][0]), 8);
        assert_eq!(rounds[1][1].play(rounds[1][0]), 1);
        assert_eq!(rounds[2][1].play(rounds[2][0]), 6);

        assert_eq!(part1(&rounds), 15);

    }

    #[test]
    fn test_part2() {
        let rounds: Vec<(Move, Outcome)> = INPUT.lines().map(|line| {
            let mut iter = line.splitn(2, ' ');
            (iter.next().unwrap().into(), iter.next().unwrap().into())
        }).collect();

        assert_eq!(rounds[0].0.guess(rounds[0].1), 4);
        assert_eq!(rounds[1].0.guess(rounds[1].1), 1);
        assert_eq!(rounds[2].0.guess(rounds[2].1),7);

        assert_eq!(part2(&rounds), 12);

   }
}
