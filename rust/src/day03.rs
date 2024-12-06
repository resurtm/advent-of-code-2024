use regex::Regex;
use std::fs;

pub fn solve() {
    solve_internal("test0", 1);
    solve_internal("test1", 2);
    solve_internal("google", -1);
    solve_internal("gh", -1);
}

fn solve_internal(test_name: &str, only_part: i32) -> (i32, i32) {
    let input = fs::read_to_string(format!("../data/day03/{}.txt", test_name))
        .expect("cannot read input file");

    let part1: i32 = if only_part == 1 || only_part == -1 {
        let re1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        re1.captures_iter(&input)
            .map(|x| {
                x.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * x.get(2).unwrap().as_str().parse::<i32>().unwrap()
            })
            .sum()
    } else {
        0
    };

    let mut part2: i32 = 0;
    if only_part == 2 || only_part == -1 {
        let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
        let mut mul = true;
        re2.captures_iter(&input).for_each(|x| {
            let op = x.get(0).unwrap().as_str();
            if op.starts_with("mul(") {
                if mul {
                    part2 += x.get(1).unwrap().as_str().parse::<i32>().unwrap()
                        * x.get(2).unwrap().as_str().parse::<i32>().unwrap();
                }
            } else if op.starts_with("do(") {
                mul = true;
            } else if op.starts_with("don't(") {
                mul = false;
            } else {
                panic!("this should not happen");
            }
        });
    }

    println!("Test Name: {}", test_name);
    println!("Day 03, Part 1: {}", part1);
    println!("Day 03, Part 2: {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0", 1).0, 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test1", 2).1, 48);
    }
}
