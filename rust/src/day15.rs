use std::fs;

pub fn solve() {
    // WarehouseWoes::new(String::from("test0")).solve();
    // WarehouseWoes::new(String::from("test1")).solve();
    WarehouseWoes::new(String::from("test2")).solve();
    // WarehouseWoes::new(String::from("test3")).solve();
    // WarehouseWoes::new(String::from("gh")).solve();
    // WarehouseWoes::new(String::from("google")).solve();
}

impl WarehouseWoes {
    fn simulate2(&mut self) {
        println!("initial");
        self.print();

        for (idx, m) in self.moves.iter().enumerate() {
            let (t, b, l, r) = self.get_sides();

            match m {
                '^' => {
                    if t == '.' {
                        self.pos.0 -= 1;
                    } else if t == '[' || t == ']' {
                        let mut int0 = if t == '[' { self.pos.1 } else { self.pos.1 - 1 };
                        let mut int1 = if t == ']' { self.pos.1 } else { self.pos.1 + 1 };
                        let mut z = self.pos.0;
                        loop {
                            z -= 1;
                            let chs = &self.map[z as usize][int0 as usize..int1 as usize + 1];
                            let all_free = chs.iter().all(|x| *x == '.');
                            let has_wall = chs.iter().any(|x| *x == '#');
                            if *chs.first().unwrap() == ']' {
                                int0 -= 1;
                            }
                            if *chs.last().unwrap() == '[' {
                                int1 += 1;
                            }
                            println!("`{:?}` {} {}", chs, chs.len(), all_free);
                            if all_free {
                                for w in z..self.pos.0 - 1 {
                                    for (int_idx, int) in (int0..int1 + 1).enumerate() {
                                        self.map[w as usize][int as usize] =
                                            if int_idx % 2 == 0 { '[' } else { ']' };
                                        self.map[(w + 1) as usize][int as usize] = '.';
                                    }
                                    int0 += 1;
                                    int1 -= 1;
                                }
                                self.pos.0 -= 1;
                                break;
                            }
                            if has_wall {
                                break;
                            }
                        }
                    }
                }
                'v' => {
                    if b == '.' {
                        self.pos.0 += 1;
                    } else if b == '[' || b == ']' {
                        let mut int0 = if t == '[' { self.pos.1 } else { self.pos.1 - 1 };
                        let mut int1 = if t == ']' { self.pos.1 } else { self.pos.1 + 1 };
                        let mut z = self.pos.0;
                        loop {
                            z += 1;
                            let chs = &self.map[z as usize][int0 as usize..int1 as usize + 1];
                            let all_free = chs.iter().all(|x| *x == '.');
                            let has_wall = chs.iter().any(|x| *x == '#');
                            if *chs.first().unwrap() == ']' {
                                int0 -= 1;
                            }
                            if *chs.last().unwrap() == '[' {
                                int1 += 1;
                            }
                            println!("`{:?}` {} {}", chs, chs.len(), all_free);
                            if all_free {
                                for w in (z..self.pos.0 - 1).rev() {
                                    for (int_idx, int) in (int0..int1 + 1).enumerate() {
                                        self.map[w as usize][int as usize] =
                                            if int_idx % 2 == 0 { '[' } else { ']' };
                                        self.map[(w + 1) as usize][int as usize] = '.';
                                    }
                                    int0 += 1;
                                    int1 -= 1;
                                }
                                self.pos.0 += 1;
                                break;
                            }
                            if has_wall {
                                break;
                            }
                        }
                    }
                }
                '<' => {
                    if l == '.' {
                        self.pos.1 -= 1;
                    } else if l == '[' || l == ']' {
                        let mut z = self.pos.1;
                        loop {
                            z -= 1;
                            let ch = self.get(self.pos.0, z);
                            if ch == '.' {
                                for w in z..self.pos.1 + 1 {
                                    self.map[self.pos.0 as usize][w as usize] =
                                        self.map[self.pos.0 as usize][(w + 1) as usize];
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.1 -= 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                '>' => {
                    if r == '.' {
                        self.pos.1 += 1;
                    } else if r == '[' || r == ']' {
                        let mut z = self.pos.1;
                        loop {
                            z += 1;
                            let ch = self.get(self.pos.0, z);
                            if ch == '.' {
                                for w in (self.pos.1..z + 1).rev() {
                                    self.map[self.pos.0 as usize][w as usize] =
                                        self.map[self.pos.0 as usize][(w - 1) as usize];
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.1 += 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                _ => panic!("bad move"),
            }

            println!("{}", m);
            self.print();
            // if idx >= 2 {
            //     break;
            // }
        }
    }

    fn simulate(&mut self) {
        for (idx, m) in self.moves.iter().enumerate() {
            let (t, b, l, r) = self.get_sides();
            // println!("{} {} {} {}", t, b, l, r);

            match m {
                '^' => {
                    if t == '.' {
                        self.pos.0 -= 1;
                    } else if t == 'O' {
                        let mut z = self.pos.0;
                        loop {
                            z -= 1;
                            let ch = self.get(z, self.pos.1);
                            if ch == '.' {
                                for w in z..self.pos.0 - 1 {
                                    self.map[w as usize][self.pos.1 as usize] = 'O';
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.0 -= 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                'v' => {
                    if b == '.' {
                        self.pos.0 += 1;
                    } else if b == 'O' {
                        let mut z = self.pos.0;
                        loop {
                            z += 1;
                            let ch = self.get(z, self.pos.1);
                            if ch == '.' {
                                for w in self.pos.0..z + 1 {
                                    self.map[w as usize][self.pos.1 as usize] = 'O';
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.0 += 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                '<' => {
                    if l == '.' {
                        self.pos.1 -= 1;
                    } else if l == 'O' {
                        let mut z = self.pos.1;
                        loop {
                            z -= 1;
                            let ch = self.get(self.pos.0, z);
                            if ch == '.' {
                                for w in z..self.pos.1 - 1 {
                                    self.map[self.pos.0 as usize][w as usize] = 'O';
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.1 -= 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                '>' => {
                    if r == '.' {
                        self.pos.1 += 1;
                    } else if r == 'O' {
                        let mut z = self.pos.1;
                        loop {
                            z += 1;
                            let ch = self.get(self.pos.0, z);
                            if ch == '.' {
                                for w in self.pos.1..z + 1 {
                                    self.map[self.pos.0 as usize][w as usize] = 'O';
                                }
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                self.pos.1 += 1;
                                self.map[self.pos.0 as usize][self.pos.1 as usize] = '.';
                                // println!("{}", z);
                                break;
                            } else if ch == '#' {
                                break;
                            }
                        }
                    }
                }
                _ => panic!("bad move"),
            }

            // println!("{}", m);
            // self.print();
            // if idx >= 20 {
            //     break;
            // }
        }
    }

    fn get_sides(&self) -> (char, char, char, char) {
        let t = self.get(self.pos.0 - 1, self.pos.1);
        let b = self.get(self.pos.0 + 1, self.pos.1);
        let l = self.get(self.pos.0, self.pos.1 - 1);
        let r = self.get(self.pos.0, self.pos.1 + 1);
        return (t, b, l, r);
    }

    fn get(&self, i: i32, j: i32) -> char {
        if i < 0 || j < 0 || i == self.w || j == self.h {
            return '#';
        }
        self.map[i as usize][j as usize]
    }

    fn calculate(&self) -> i32 {
        let mut ret = 0;
        for i in 0..self.w {
            for j in 0..self.h {
                if self.get(i, j) == 'O' {
                    ret += 100 * i + j;
                }
            }
        }
        ret
    }

    fn solve(&mut self) -> (i32, i32) {
        self.reset();
        self.set_start();
        self.simulate();
        self.part1 = self.calculate();

        self.reset();
        self.widen();
        self.set_start();
        self.simulate2();

        // println!("Map:\n{:?}", self.map);
        // println!("Moves:\n{:?}", self.moves);

        println!("Test Name: {}", self.test_name);
        println!("Day 15, Part 1: {}", self.part1);
        println!("Day 15, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn set_start(&mut self) {
        for i in 0..self.w {
            for j in 0..self.h {
                if self.map[i as usize][j as usize] == '@' {
                    self.map[i as usize][j as usize] = '.';
                    self.pos = (i, j);
                    return;
                }
            }
        }
        panic!("start cannot be found")
    }

    fn print(&self) {
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

    fn widen(&mut self) {
        for i in 0..self.w {
            let mut new = Vec::new();
            for j in 0..self.h {
                match self.map[i as usize][j as usize] {
                    '#' => {
                        new.push('#');
                        new.push('#');
                    }
                    'O' => {
                        new.push('[');
                        new.push(']');
                    }
                    '.' => {
                        new.push('.');
                        new.push('.');
                    }
                    '@' => {
                        new.push('@');
                        new.push('.');
                    }
                    _ => panic!("bad char"),
                }
            }
            self.map[i as usize] = new;
        }
        self.h = self.map[0].len() as i32;
    }

    fn reset(&mut self) {
        self.map = self.mapi.clone();
    }

    fn read_input(test_name: &str) -> (Vec<Vec<char>>, Vec<char>) {
        let raw =
            fs::read_to_string(format!("../data/day15/{}.txt", test_name)).expect("input file");
        let parts: Vec<&str> = raw.split("\n\n").collect();

        let map = parts
            .first()
            .unwrap()
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect())
            .collect();
        let moves = parts
            .get(1)
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .collect();

        (map, moves)
    }

    fn new(test_name: String) -> WarehouseWoes {
        let (mapi, moves) = Self::read_input(&test_name);
        let w = mapi.len() as i32;
        let h = mapi[0].len() as i32;
        WarehouseWoes {
            mapi,
            map: Vec::new(),
            moves,
            pos: (0, 0),
            w,
            h,
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct WarehouseWoes {
    mapi: Vec<Vec<char>>,
    map: Vec<Vec<char>>,
    moves: Vec<char>,
    pos: (i32, i32),
    w: i32,
    h: i32,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::WarehouseWoes;

    #[test]
    fn test_part1() {
        assert_eq!(WarehouseWoes::new(String::from("test0")).solve().0, 2028);
        assert_eq!(WarehouseWoes::new(String::from("test1")).solve().0, 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(WarehouseWoes::new(String::from("test0")).solve().1, 0);
    }
}
