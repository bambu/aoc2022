fn main() {
    let input = include_str!("../../inputs/input1.txt");
    println!("Day1 Part1: {}", day1_part1(input));
    println!("Day1 Part2: {}", day1_part2(input));
}

fn day1_part1(input: &str) -> usize {
    let mut most_calories = 0;
    let mut cur_sum = 0;

    for line in input.lines() {
        match line {
            "" => {
                most_calories = most_calories.max(cur_sum);
                cur_sum = 0;
            }
            _ => cur_sum += line.parse::<usize>().unwrap(),
        }
    }

    most_calories.max(cur_sum)
}

fn day1_part2(input: &str) -> usize {
    let mut calories = Vec::new();
    let mut cur_sum = 0;

    for line in input.lines() {
        match line {
            "" => {
                calories.push(cur_sum);
                cur_sum = 0;
            }
            _ => cur_sum += line.parse::<usize>().unwrap(),
        }
    }

    calories.push(cur_sum);

    calories.sort_unstable();
    calories.reverse();

    calories[..3].iter().sum()
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_test() {
        assert_eq!(day1_part1(INPUT), 24000);
    }

    #[test]
    fn part2_test() {
        assert_eq!(day1_part2(INPUT), 45000);
    }
}
