use regex::Regex;
use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let input = fs::read_to_string(format!("../data/day03/{}.txt", test_name))
        .expect("cannot read input file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let part1: i32 = re
        .captures_iter(&input)
        .map(|x| {
            x.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * x.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum();
    let part2: i32 = 0;

    println!("Test Name: {}", test_name);
    println!("Day 01, Part 1: {}", part1);
    println!("Day 01, Part 2: {}", part2);

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 0);
    }
}
