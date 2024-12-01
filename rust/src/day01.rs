use std::{collections::HashMap, fs};

pub fn solve() {
    solve_internal("test0");
    solve_internal("google");
    solve_internal("gh");
}

fn solve_internal(test_name: &str) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    fs::read_to_string(format!("../data/day01/{}.txt", test_name))
        .expect("cannot read input file")
        .split("\n")
        .into_iter()
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| {
            let parts = x
                .split(" ")
                .into_iter()
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<i32>().expect("invalid data type"))
                .collect::<Vec<i32>>();
            (
                *parts.get(0).expect("invalid data"),
                *parts.get(1).expect("invalid data"),
            )
        })
        .for_each(|(x, y)| {
            a.push(x);
            b.push(y);
        });

    a.sort();
    b.sort();
    let mut c = Vec::new();
    let mut freq = HashMap::new();

    for idx in 0..a.len() {
        c.push((a[idx] - b[idx]).abs());
        if !freq.contains_key(&b[idx]) {
            freq.insert(b[idx], 1);
        } else {
            freq.insert(b[idx], *freq.get(&b[idx]).expect("not expected") + 1);
        }
    }

    let part1 = c.iter().sum::<i32>();
    let part2 = a.iter().map(|x| x * freq.get(x).unwrap_or(&0)).sum::<i32>();

    println!("Test name: {}", test_name);
    println!("Day 01, part 1: {}", part1);
    println!("Day 01, part 2: {}", part2);
}
