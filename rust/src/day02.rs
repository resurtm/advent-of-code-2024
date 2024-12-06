use std::fs;

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) -> (i32, i32) {
    let lines: Vec<Vec<i32>> = fs::read_to_string(format!("../data/day02/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .into_iter()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.split(" ")
                .into_iter()
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<i32>().expect("invalid data type"))
                .collect::<Vec<i32>>()
        })
        .collect();

    let part1 = lines
        .iter()
        .map(|x| is_decr(&x) || is_incr(&x))
        .fold(0, |acc, x| if x { acc + 1 } else { acc });

    let part2 = lines
        .iter()
        .map(|x| is_fixable(&x))
        .fold(0, |acc, x| if x { acc + 1 } else { acc });

    println!("Test Name: {}", test_name);
    println!("Day 02, Part 1: {}", part1);
    println!("Day 02, Part 2: {}", part2);

    (part1, part2)
}

fn is_fixable(v: &Vec<i32>) -> bool {
    for rem_idx in 0..v.len() {
        let mut tmp = v.clone();
        tmp.remove(rem_idx);
        if is_decr(&tmp) || is_incr(&tmp) {
            return true;
        }
    }
    false
}

fn is_decr(v: &Vec<i32>) -> bool {
    for idx in 0..v.len() - 1 {
        let diff = v[idx] - v[idx + 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_incr(v: &Vec<i32>) -> bool {
    for idx in 0..v.len() - 1 {
        let diff = v[idx + 1] - v[idx];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::solve_internal;

    #[test]
    fn test_part1() {
        assert_eq!(solve_internal("test0").0, 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_internal("test0").1, 4);
    }
}
