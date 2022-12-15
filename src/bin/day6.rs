use std::collections::HashSet;

const DAY_NUM: &str = "6";

fn main() {
    let input = include_str!("../../inputs/input6.txt");
    println!("Day{} Part1: {}", DAY_NUM, part1(input));
    println!("Day{} Part2: {}", DAY_NUM, part2(input));
}

fn find_uniq_offset(stream: &str, min_unique: usize) -> usize {
    let data = stream.chars().collect::<Vec<char>>();
    let mut start = 0;
    for (i, window) in data.windows(min_unique).enumerate(){
        if window.iter().collect::<HashSet<&char>>().len() == min_unique {
            start = i + min_unique;
            break;
        }
    }
    start

}

fn part1(stream: &str) -> usize {
    find_uniq_offset(stream, 4)
}

fn part2(stream: &str) -> usize{
    find_uniq_offset(stream, 14)
}



#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        let inputs = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(part1(inputs.get(0).unwrap()), 7);
        assert_eq!(part1(inputs.get(1).unwrap()), 5);
        assert_eq!(part1(inputs.get(2).unwrap()), 6);
        assert_eq!(part1(inputs.get(3).unwrap()), 10);
        assert_eq!(part1(inputs.get(4).unwrap()), 11);
    }

    #[test]
    fn test_part2() {
        let inputs = INPUT.lines().collect::<Vec<&str>>();
        assert_eq!(part2(inputs.get(0).unwrap()), 19);
        assert_eq!(part2(inputs.get(1).unwrap()), 23);
        assert_eq!(part2(inputs.get(2).unwrap()), 23);
        assert_eq!(part2(inputs.get(3).unwrap()), 29);
        assert_eq!(part2(inputs.get(4).unwrap()), 26);
    }
}
