use std::fs;

pub fn solve() {
    DiskFragmenter::new(String::from("test0")).solve();
    DiskFragmenter::new(String::from("gh")).solve();
    // DiskFragmenter::new(String::from("google")).solve();
}

impl DiskFragmenter {
    fn new(test_name: String) -> DiskFragmenter {
        DiskFragmenter {
            input: Self::read_input(&test_name),
            part1: 0,
            part2: 0,
            test_name,
        }
    }

    fn solve_internal(&self) -> i128 {
        // step 1
        let mut map = String::new();
        {
            let mut index = 0;
            let mut state = false;
            for ch in self.input.chars() {
                if !ch.is_alphanumeric() {
                    continue;
                }
                let times = ch.to_digit(10).unwrap();
                let st = if state {
                    ".".to_string()
                } else {
                    index.to_string()
                };
                map.push_str(&st.repeat(times as usize));
                if !state {
                    index += 1;
                }
                state = !state;
            }
        }

        // step 2
        let mut map2 = String::new();
        {
            let last = map.chars().filter(|x| *x != '.').rev().collect::<String>();
            let mut last_pos = 0;
            for ch in map.chars() {
                if ch == '.' {
                    map2.push(last.chars().nth(last_pos).unwrap());
                    last_pos += 1;
                } else {
                    map2.push(ch);
                }
            }
        }

        // step 3
        let dot_count = map.chars().filter(|x| *x == '.').count();
        // map2.replace_range(map2.len() - dot_count..map2.len(), &".".repeat(dot_count));
        println!("{}", map2.len());
        map2.replace_range(map2.len() - dot_count..map2.len(), "");
        println!("{}", map2.len());

        // step 4
        let mut res = 0;
        for (i, ch) in map2.chars().enumerate() {
            res += i as i128 * ch.to_digit(10).unwrap_or(0) as i128;
        }
        res
    }

    fn solve(&mut self) -> (i128, i128) {
        self.part1 = self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 09, Part 1: {}", self.part1);
        println!("Day 09, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> String {
        fs::read_to_string(format!("../data/day09/{}.txt", test_name)).expect("input file")
    }
}

struct DiskFragmenter {
    input: String,
    part1: i128,
    part2: i128,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::DiskFragmenter;

    #[test]
    fn test_part1() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().0, 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(DiskFragmenter::new(String::from("test0")).solve().1, 0);
    }
}
