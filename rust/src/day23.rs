use std::{collections::HashSet, fs};

pub fn solve() {
    LanParty::new(String::from("test0")).solve();
    LanParty::new(String::from("gh")).solve();
    LanParty::new(String::from("google")).solve();
}

impl LanParty {
    fn solve_internal(&self) -> i32 {
        let mut res = 0;
        for i in 0..self.comps.len() {
            for j in i + 1..self.comps.len() {
                for k in j + 1..self.comps.len() {
                    let m = self.comps.get(i).unwrap().clone();
                    let n = self.comps.get(j).unwrap().clone();
                    let p = self.comps.get(k).unwrap().clone();
                    if self.conns.contains(&(m.clone(), n.clone()))
                        && self.conns.contains(&(n.clone(), p.clone()))
                        && self.conns.contains(&(m.clone(), p.clone()))
                        && (m.starts_with("t") || n.starts_with("t") || p.starts_with("t"))
                    {
                        res += 1;
                    }
                }
            }
        }
        res
    }

    fn solve(&mut self) -> (i32, i32) {
        self.part1 = self.solve_internal();

        println!("Test Name: {}", self.test_name);
        println!("Day 22, Part 1: {}", self.part1);
        println!("Day 22, Part 2: {}", self.part2);

        (self.part1, self.part2)
    }

    fn read_input(test_name: &str) -> Vec<(String, String)> {
        let raw =
            fs::read_to_string(format!("../data/day23/{}.txt", test_name)).expect("input file");
        raw.lines()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let parts: Vec<&str> = x.split("-").collect();
                (parts[0].to_owned(), parts[1].to_owned())
            })
            .collect()
    }

    fn new(test_name: String) -> LanParty {
        let input = Self::read_input(&test_name);
        let mut conns = HashSet::new();
        let mut comps = HashSet::new();
        input.iter().for_each(|(a, b)| {
            conns.insert((a.clone(), b.clone()));
            conns.insert((b.clone(), a.clone()));
            comps.insert(a.clone());
            comps.insert(b.clone());
        });
        LanParty {
            input,
            conns,
            comps: comps.into_iter().collect(),
            part1: 0,
            part2: 0,
            test_name,
        }
    }
}

struct LanParty {
    input: Vec<(String, String)>,
    conns: HashSet<(String, String)>,
    comps: Vec<String>,
    part1: i32,
    part2: i32,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::LanParty;

    #[test]
    fn test_part1() {
        assert_eq!(LanParty::new(String::from("test0")).solve().0, 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(LanParty::new(String::from("test1")).solve().1, 0);
    }
}
