use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    state: Vec<i64>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp { state: Vec::new() };

    let mut line = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for n in line.split(" ") {
        res.state.push(n.parse::<i64>().unwrap());
    }
    return res;
}

impl Inp {
    fn step(&mut self) {
        let mut i = 0;
        while i < self.state.len() {
            //println!("(i) {} {:?}", i, self.state);
            if self.state[i] == 0 {
                self.state[i] = 1;
                i += 1;
                continue;
            }
            let l = self.state[i].to_string().len();
            if l % 2 == 0 {
                let s1 = self.state[i].to_string()[0..l / 2].parse::<i64>().unwrap();
                let s2 = self.state[i].to_string()[l / 2..].parse::<i64>().unwrap();
                self.state[i] = s1;
                self.state.insert(i + 1, s2);
                i += 2;
                continue;
            }
            self.state[i] = 2024 * self.state[i];
            i += 1;
        }
    }
    fn metrics(&mut self) -> i64 {
        for i in 0..25 {
            println!("{} {}", i, self.state.len());
            self.step();
        }
        return self.state.len() as i64;
    }
    fn len(&self, v: i64, rem_depth: i64, h: &mut HashMap<(i64, i64), i64>) -> i64 {
        if h.contains_key(&(v, rem_depth)) {
            return *h.get(&(v, rem_depth)).unwrap();
        }
        if rem_depth == 0 {
            return 1;
        } else {
            if v == 0 {
                let res = self.len(1, rem_depth - 1, h);
                h.insert((v, rem_depth), res);
                return res;
            }
            let l = v.to_string().len();
            if l % 2 == 0 {
                let s1 = v.to_string()[0..l / 2].parse::<i64>().unwrap();
                let s2 = v.to_string()[l / 2..].parse::<i64>().unwrap();
                let res = self.len(s1, rem_depth - 1, h) + self.len(s2, rem_depth - 1, h);
                h.insert((v, rem_depth), res);
                return res;
            }
            let res = self.len(2024 * v, rem_depth - 1, h);
            h.insert((v, rem_depth), res);
            return res;
        }
    }

    fn metrics2(&mut self) -> i64 {
        let mut s = 0;
        let mut h: HashMap<(i64, i64), i64> = HashMap::new();
        let mut i = 0;
        for v in &self.state {
            i += 1;
            println!("{} {}", i, s);
            s += self.len(*v, 50, &mut h);
        }
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 55312);
        assert_eq!(read_file("test2.txt").metrics2(), 0);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
