use std::{collections::HashSet, fs};

pub fn solve() {
    LanParty::new(String::from("test0")).solve();
    // LanParty::new(String::from("test1")).solve();
    LanParty::new(String::from("gh")).solve();
    LanParty::new(String::from("google")).solve();
}

impl LanParty {
    fn solve_internal_part1(&self) -> i32 {
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

    fn compare(&self, a: &HashSet<(String, String)>, b: &HashSet<(String, String)>) -> bool {
        let mut comps = HashSet::new();
        a.iter().for_each(|x| {
            comps.insert(x.0.clone());
            comps.insert(x.1.clone());
        });
        b.iter().for_each(|x| {
            comps.insert(x.0.clone());
            comps.insert(x.1.clone());
        });
        let comps: Vec<_> = comps.into_iter().collect();

        for i in 0..comps.len() {
            for j in i + 1..comps.len() {
                let m = comps.get(i).unwrap().clone();
                let n = comps.get(j).unwrap().clone();
                if !self.conns.contains(&(m.clone(), n.clone())) {
                    return false;
                }
            }
        }

        true
    }

    fn solve_internal_part2(&mut self) -> String {
        // let res = self.compare(
        //     &HashSet::from([(String::from("ka"), String::from("co"))]),
        //     &HashSet::from([(String::from("ta"), String::from("co"))]),
        // );
        // println!("{}", res);
        // return 0;

        let mut i = 0;
        loop {
            if i == self.nets.len() {
                break;
            }
            let mut net = self.nets.get_mut(i).unwrap().clone();
            let mut j = i + 1;
            loop {
                if j == self.nets.len() {
                    break;
                }
                let other_net = self.nets.get(j).unwrap().clone();
                if self.compare(&net, &other_net) {
                    net.extend(other_net.clone());
                    self.nets[i] = net.clone();
                    self.nets.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        let mut max = 0;
        let mut imax = 0;
        for (idx, net) in self.nets.iter().enumerate() {
            if max == 0 && imax == 0 || max < net.len() {
                max = net.len();
                imax = idx;
            }
        }
        let mut res = HashSet::new();
        for it in self.nets[imax].iter() {
            res.insert(it.0.clone());
            res.insert(it.1.clone());
        }
        let mut res: Vec<_> = res.into_iter().collect();
        res.sort();
        // println!("{}", res.join(","));
        res.join(",")
    }

    fn solve(&mut self) -> (i32, String) {
        self.part1 = self.solve_internal_part1();
        self.part2 = self.solve_internal_part2();

        println!("Test Name: {}", self.test_name);
        println!("Day 23, Part 1: {}", self.part1);
        println!("Day 23, Part 2: {}", self.part2);

        (self.part1, self.part2.clone())
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
        let mut nets = Vec::new();

        input.iter().for_each(|(a, b)| {
            conns.insert((a.clone(), b.clone()));
            conns.insert((b.clone(), a.clone()));

            comps.insert(a.clone());
            comps.insert(b.clone());

            let mut tmp = HashSet::new();
            // tmp.insert((a.clone(), b.clone()));
            tmp.insert((b.clone(), a.clone()));
            nets.push(tmp);
        });

        LanParty {
            input,
            conns,
            comps: comps.into_iter().collect(),
            nets,
            part1: 0,
            part2: String::new(),
            test_name,
        }
    }
}

struct LanParty {
    input: Vec<(String, String)>,
    conns: HashSet<(String, String)>,
    comps: Vec<String>,
    nets: Vec<HashSet<(String, String)>>,
    part1: i32,
    part2: String,
    test_name: String,
}

#[cfg(test)]
mod tests {
    use super::LanParty;

    #[test]
    fn test_part1() {
        assert_eq!(LanParty::new(String::from("test0")).solve().0, 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            LanParty::new(String::from("test0")).solve().1,
            String::from("co,de,ka,ta")
        );
    }
}
