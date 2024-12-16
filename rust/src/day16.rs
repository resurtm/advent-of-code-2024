use std::fs;

pub fn solve() {
    ReindeerMaze::new(String::from("test0")).solve();
    // ReindeerMaze::new(String::from("gh")).solve();
    // ReindeerMaze::new(String::from("google")).solve();
}

impl ReindeerMaze {
    fn solve(&mut self) -> (i32, i32) {
        self.print();

        println!("Test Name: {}", self.test_name);
        println!("Day 16, Part 1: {}", self.part1);
        println!("Day 16, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn print(&self) {
        println!("{}", "-".repeat(self.h as usize));
        for i in 0..self.w {
            for j in 0..self.h {
                print!(
                    "{}",
                    if self.pos == (i, j) {
                        '@'
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
    start: (i32, i32),
    end: (i32, i32),
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::ReindeerMaze;

    #[test]
    fn test_part1() {
        assert_eq!(ReindeerMaze::new(String::from("test0")).solve().0, 0);
        assert_eq!(ReindeerMaze::new(String::from("test1")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(ReindeerMaze::new(String::from("test0")).solve().1, 0);
        assert_eq!(ReindeerMaze::new(String::from("test1")).solve().1, 0);
    }
}
