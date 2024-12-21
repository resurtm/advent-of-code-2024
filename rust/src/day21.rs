use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn solve() {
    KeypadConundrum::new(String::from("test0")).solve();
    KeypadConundrum::new(String::from("gh")).solve();
    KeypadConundrum::new(String::from("google")).solve();
}

impl KeypadConundrum {
    fn bfs_keypad1(
        &self,
        src: char,
        dst: char,
        cache1: &mut HashMap<(char, char), Vec<String>>,
    ) -> Vec<String> {
        if let Some(existing) = cache1.get(&(src, dst)) {
            return existing.clone();
        }

        let mut routes: Vec<Vec<(i32, i32)>> = vec![];

        let mut queue: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
        queue.push_back(vec![self.keypad1[&src]]);

        while !queue.is_empty() {
            if let Some(route) = queue.pop_front() {
                if let Some(&last) = route.last() {
                    if self.keypad1[&dst] == last {
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
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= 2 && *y <= 1)
                    .filter(|(x, y)| !(*x == 0 && *y == 0))
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
                res.push(Self::route_to_chars(route));
            }
        }
        cache1.insert((src, dst), res.clone());
        res
    }

    fn bfs_keypad0(
        &self,
        src: char,
        dst: char,
        cache0: &mut HashMap<(char, char), Vec<String>>,
    ) -> Vec<String> {
        if let Some(existing) = cache0.get(&(src, dst)) {
            return existing.clone();
        }

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
                    .filter(|(x, y)| !(*x == 0 && *y == 3))
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
                res.push(Self::route_to_chars(route));
            }
        }
        cache0.insert((src, dst), res.clone());
        res
    }

    fn route_to_chars(route: &[(i32, i32)]) -> String {
        let mut res = String::from("A");
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

    fn launch1(
        &self,
        item: String,
        cache0: &mut HashMap<(char, char), Vec<String>>,
        cache1: &mut HashMap<(char, char), Vec<String>>,
        cache2: &mut HashMap<(String, i128), i128>,
        depth: i128,
    ) -> i128 {
        if let Some(existing) = cache2.get(&(item.clone(), depth)) {
            return *existing;
        }
        if depth == 0 {
            return item.len() as i128 - 1;
        }
        let mut cost = 0;
        let chs = item.chars().collect::<Vec<char>>();
        for idx in 0..chs.len() - 1 {
            let mut min = 0;
            let subs = self.bfs_keypad1(chs[idx], chs[idx + 1], cache0);
            for sub in subs.iter() {
                let sub_curr = self.launch1(sub.clone(), cache0, cache1, cache2, depth - 1);
                if min == 0 || min > sub_curr {
                    min = sub_curr;
                }
            }
            cost += min;
        }
        cache2.insert((item.clone(), depth), cost);
        cost
    }

    fn launch0(
        &self,
        itemr: String,
        cache0: &mut HashMap<(char, char), Vec<String>>,
        cache1: &mut HashMap<(char, char), Vec<String>>,
        cache2: &mut HashMap<(String, i128), i128>,
        depth: i128,
    ) -> i128 {
        let mut item = itemr;
        item.insert_str(0, "A");
        let mut cost = 0;
        let chs = item.chars().collect::<Vec<char>>();
        for idx in 0..chs.len() - 1 {
            let mut min = 0;
            let subs = self.bfs_keypad0(chs[idx], chs[idx + 1], cache0);
            for sub in subs.iter() {
                let sub_curr = self.launch1(sub.clone(), cache0, cache1, cache2, depth);
                if min == 0 || min > sub_curr {
                    min = sub_curr;
                }
            }
            cost += min;
        }
        cost
    }

    fn solve(&mut self) -> (i128, i128) {
        let mut cache0: HashMap<(char, char), Vec<String>> = HashMap::new();
        let mut cache1: HashMap<(char, char), Vec<String>> = HashMap::new();
        let mut cache2: HashMap<(String, i128), i128> = HashMap::new();

        for it in self.input.iter() {
            let a = self.launch0(it.clone(), &mut cache0, &mut cache1, &mut cache2, 2);
            let b = it
                .chars()
                .filter(|x| '0' <= *x && *x <= '9')
                .collect::<String>()
                .parse::<i128>()
                .unwrap_or(0);
            self.part1 += a * b;
        }

        for it in self.input.iter() {
            let a = self.launch0(it.clone(), &mut cache0, &mut cache1, &mut cache2, 25);
            let b = it
                .chars()
                .filter(|x| '0' <= *x && *x <= '9')
                .collect::<String>()
                .parse::<i128>()
                .unwrap_or(0);
            self.part2 += a * b;
        }

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
            cache0: HashMap::new(),

            keypad1,
            cache1: HashMap::new(),

            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct KeypadConundrum {
    input: Vec<String>,

    keypad0: HashMap<char, (i32, i32)>,
    cache0: HashMap<(char, char), Vec<String>>,

    keypad1: HashMap<char, (i32, i32)>,
    cache1: HashMap<(char, char), Vec<String>>,

    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::KeypadConundrum;

    #[test]
    fn test_part1() {
        assert_eq!(
            KeypadConundrum::new(String::from("test0")).solve().0,
            126_384
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            KeypadConundrum::new(String::from("test0")).solve().1,
            154_115_708_116_294
        );
    }
}
