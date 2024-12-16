use std::{
    collections::{HashSet, VecDeque},
    fs, usize,
};

pub fn solve() {
    ReindeerMaze::new(String::from("test0")).solve();
    ReindeerMaze::new(String::from("test1")).solve();
    ReindeerMaze::new(String::from("gh")).solve();
    // ReindeerMaze::new(String::from("google")).solve();
}

impl ReindeerMaze {
    fn simulate_bfs(&self) -> Vec<(i32, i32, Direction)> {
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        let mut viz = HashSet::new();

        let temp = (self.pos.0, self.pos.1, self.dir.clone());
        q.push_back(vec![temp.clone()]);
        viz.insert(temp);

        loop {
            let curr = q.pop_front().unwrap();
            let last = curr.last().unwrap();
            if last.0 == self.end.0 && last.1 == self.end.1 {
                // println!("found route: {:?}", curr.len());
                if res.len() == 0 || res.len() < curr.len() {
                    res = curr;
                }
                continue;
            }
            for next in self.get_actions(last) {
                if viz.contains(&next) {
                    continue;
                }
                let mut new_item = curr.clone();
                new_item.push(next.clone());
                q.push_back(new_item);
                viz.insert(next);
            }
            if q.is_empty() {
                break;
            }
        }

        res
    }

    fn get_actions(&self, curr: &(i32, i32, Direction)) -> Vec<(i32, i32, Direction)> {
        let mut res = Vec::new();
        match curr.2 {
            Direction::North | Direction::South => {
                res.push((curr.0, curr.1, Direction::West));
                res.push((curr.0, curr.1, Direction::East));
            }
            Direction::West | Direction::East => {
                res.push((curr.0, curr.1, Direction::North));
                res.push((curr.0, curr.1, Direction::South));
            }
        }
        match curr.2 {
            Direction::North => {
                if self.map[(curr.0 - 1) as usize][curr.1 as usize] != '#' {
                    res.push((curr.0 - 1, curr.1, Direction::North));
                }
            }
            Direction::South => {
                if self.map[(curr.0 + 1) as usize][curr.1 as usize] != '#' {
                    res.push((curr.0 + 1, curr.1, Direction::South));
                }
            }
            Direction::West => {
                if self.map[curr.0 as usize][(curr.1 - 1) as usize] != '#' {
                    res.push((curr.0, curr.1 - 1, Direction::West));
                }
            }
            Direction::East => {
                if self.map[curr.0 as usize][(curr.1 + 1) as usize] != '#' {
                    res.push((curr.0, curr.1 + 1, Direction::East));
                }
            }
        }
        res
    }

    fn score(route: &[(i32, i32, Direction)]) -> i32 {
        let mut res = 0;
        for i in 0..route.len() - 1 {
            res += if route[i].2 == route[i + 1].2 {
                1
            } else {
                1_000
            };
        }
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        let route = self.simulate_bfs();
        // self.print(&route);
        self.part1 = Self::score(&route);

        println!("Test Name: {}", self.test_name);
        println!("Day 16, Part 1: {}", self.part1);
        println!("Day 16, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn print(&self, route: &[(i32, i32, Direction)]) {
        let viz: HashSet<(i32, i32)> = route.iter().map(|x| (x.0, x.1)).collect();
        println!("{}", "-".repeat(self.h as usize));
        for i in 0..self.w {
            for j in 0..self.h {
                print!(
                    "{}",
                    if viz.contains(&(i, j)) {
                        'X'
                    } else {
                        self.map[i as usize][j as usize]
                    }
                );
            }
            println!();
        }
        println!("{}", "-".repeat(self.h as usize));
    }

    fn read_input(test_name: &str) -> Vec<Vec<char>> {
        let raw =
            fs::read_to_string(format!("../data/day16/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect())
            .collect()
    }

    fn find_start_end(map: &mut [Vec<char>]) -> ((i32, i32), (i32, i32)) {
        let mut start = (0, 0);
        let mut end = (0, 0);
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == 'S' {
                    start = (i as i32, j as i32);
                    map[i][j] = '.';
                }
                if map[i][j] == 'E' {
                    end = (i as i32, j as i32);
                    map[i][j] = '.';
                }
            }
        }
        (start, end)
    }

    fn new(test_name: String) -> ReindeerMaze {
        let mut map = Self::read_input(&test_name);
        let (start, end) = Self::find_start_end(&mut map);
        let w = map.len() as i32;
        let h = map[0].len() as i32;
        ReindeerMaze {
            map,
            w,
            h,
            pos: (start.0, start.1),
            dir: Direction::East,
            start,
            end,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct ReindeerMaze {
    map: Vec<Vec<char>>,
    w: i32,
    h: i32,
    pos: (i32, i32),
    dir: Direction,
    start: (i32, i32),
    end: (i32, i32),
    part1: i32,
    part2: i32,
    test_name: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[cfg(test)]
mod tests {
    use super::ReindeerMaze;

    #[test]
    fn test_part1() {
        assert_eq!(ReindeerMaze::new(String::from("test0")).solve().0, 7_036);
        assert_eq!(ReindeerMaze::new(String::from("test1")).solve().0, 11_048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(ReindeerMaze::new(String::from("test0")).solve().1, 0);
        assert_eq!(ReindeerMaze::new(String::from("test1")).solve().1, 0);
    }
}
