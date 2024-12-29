// use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Equation {
    target: i64,
    values: Vec<i64>,
}

impl Equation {
    fn is_calculable(&self) -> bool {
        let mut opv: u16 = 0;
        for opv in 0..(1 << self.values.len()) {
            let mut res = self.values[0];
            for i in 1..self.values.len() {
                if ( opv >> (i-1) ) & 1 == 1 {
                    res *= self.values[i];
                } else {
                    res += self.values[i];
                }
            }
            if res == self.target {
                return true;
            }
        }
        return false;
    }
    fn is_calculable2(&self) -> bool {
        let opvmax = (3 as i64).pow(self.values.len() as u32 -1);
        for opv in 0..opvmax {
            let mut res = self.values[0];
            let mut oprem = opv;
            for i in 1..self.values.len() {
                if oprem % 3 == 0 {
                    res *= self.values[i];
                } else if oprem % 3 == 1 {
                    res += self.values[i];
                } else {
                    let mut base: i64 = 10;
                    base = base.pow(self.values[i].to_string().len() as u32);
                    res = res * base + self.values[i];
                }
                oprem /= 3;
            }
            if res == self.target {
                return true;
            }
        };
        return false;
    }

    fn parse(line: &str) -> Equation {
        let mut res = Equation {
            target: 0,
            values: Vec::new(),
        };
        let re = Regex::new(r"([0-9]+): ([0-9 ]+)").unwrap();
        re.captures(line).map(|c| {
            res.target = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            res.values = c.get(2).unwrap().as_str().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        });
        return res;
    }
}


#[derive(Debug)]
struct Inp {
    e: Vec<Equation>
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp {
        e: Vec::new(),
    };
    let line = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for l in line.lines() {
        inp.e.push(Equation::parse(l));
    }
    return inp;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut res = 0;
        for e in &self.e {
            if e.is_calculable() {
                res += e.target;
            }
        }
       return res;
    }
    fn metrics2(&self) -> i64 {
        let mut res = 0;
        for e in &self.e {
            if e.is_calculable2() {
                res += e.target;
            }
        }
       return res;
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 3749);
        assert_eq!(read_file("test.txt").metrics2(), 11387);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
