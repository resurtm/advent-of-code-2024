use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn solve() {
    KeypadConundrum::new(String::from("test0")).solve();
    // KeypadConundrum::new(String::from("gh")).solve();
    // KeypadConundrum::new(String::from("google")).solve();
}

impl KeypadConundrum {
    fn bfs(&self, src: char, dst: char) -> Vec<Vec<(i32, i32)>> {
        let mut routes: Vec<Vec<(i32, i32)>> = vec![];

        let mut queue: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
        queue.push_back(vec![self.keypad0[&src]]);

        while !queue.is_empty() {
            if let Some(route) = queue.pop_front() {
                if let Some(&last) = route.last() {
                    if self.keypad0[&dst] == last {
                        routes.push(route);
                        continue;
                    }
                    [
                        (last.0 - 1, last.1),
                        (last.0 + 1, last.1),
                        (last.0, last.1 - 1),
                        (last.0, last.1 + 1),
                    ]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= 2 && *y <= 3)
                    .filter(|x| !route.contains(x))
                    .for_each(|x| {
                        let mut new_route = route.clone();
                        new_route.push(*x);
                        queue.push_back(new_route);
                    });
                }
            }
        }

        let min_route = routes.iter().map(|x| x.len()).min().unwrap_or(0);
        let mut res = vec![];
        for route in routes.iter() {
            if route.len() == min_route {
                res.push(route.clone());
            }
        }
        res
    }

    fn solve_internal(&self, items: &str) {
        let mut routes: Vec<Vec<(i32, i32)>> = vec![];
        let mut curr_item = 'A';

        for item in items.chars() {
            let calc_routes = self.bfs(curr_item, item);

            let mut next_routes = vec![];
            for calc_route in calc_routes.iter() {
                if routes.is_empty() {
                    next_routes.push(calc_route.clone());
                } else {
                    for route in routes.iter() {
                        let mut next_route = route.clone();
                        next_route.extend(calc_route);
                        next_routes.push(next_route);
                    }
                }
            }
            routes = next_routes;

            curr_item = item;
        }

        for route in routes.iter() {
            println!("{} - {}", route.len(), Self::route_to_chars(route));
        }
    }

    fn route_to_chars(route: &[(i32, i32)]) -> String {
        let mut res = String::new();
        for idx in 0..route.len() - 1 {
            let curr = route[idx];
            let next = route[idx + 1];
            if curr.0 - next.0 > 0 {
                res.push('<');
            } else if curr.0 - next.0 < 0 {
                res.push('>');
            } else if curr.1 - next.1 > 0 {
                res.push('^');
            } else if curr.1 - next.1 < 0 {
                res.push('v');
            } else {
                res.push('A');
            }
        }
        res.push('A');
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        self.solve_internal("029A");

        println!("Test Name: {}", self.test_name);
        println!("Day 21, Part 1: {}", self.part1);
        println!("Day 21, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<String> {
        let raw =
            fs::read_to_string(format!("../data/day21/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned())
            .collect()
    }

    fn new(test_name: String) -> KeypadConundrum {
        let input = Self::read_input(&test_name);
        let keypad0 = HashMap::from([
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            (' ', (0, 3)),
            ('0', (1, 3)),
            ('A', (2, 3)),
        ]);
        let keypad1 = HashMap::from([
            (' ', (0, 0)),
            ('^', (1, 0)),
            ('A', (2, 0)),
            ('<', (0, 1)),
            ('v', (1, 1)),
            ('>', (2, 1)),
        ]);

        KeypadConundrum {
            input,
            keypad0,
            keypad1,

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct KeypadConundrum {
    input: Vec<String>,
    keypad0: HashMap<char, (i32, i32)>,
    keypad1: HashMap<char, (i32, i32)>,

    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::KeypadConundrum;

    #[test]
    fn test_part1() {
        assert_eq!(KeypadConundrum::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(KeypadConundrum::new(String::from("test0")).solve().1, 0);
    }
}
