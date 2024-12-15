// use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Case {
    Ax : i32,
    Ay : i32,
    Bx : i32,
    By : i32,
    Targetx: i32,
    Targety: i32,
}
#[derive(Debug)]
struct Inp {
    cases: Vec<Case>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp {
        cases: vec![],
    };
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let blocks = buf.split("\n\n");
    // Regexp to parse
    // "Button A: X+94, Y+34"
    let r1 = Regex::new(r"Button A: X\+([0-9]{1,3}), Y\+([0-9]{1,3})").unwrap();
    let r2 = Regex::new(r"Button B: X\+([0-9]{1,3}), Y\+([0-9]{1,3})").unwrap();
    // Regexp to parse
    // Prize: X=7870, Y=6450
    let r3 = Regex::new(r"Prize: X=([0-9]{1,5}), Y=([0-9]{1,5})").unwrap();
    for block in blocks {
        let lines = block.lines().collect::<Vec<&str>>();
        if r1.is_match(lines[0]) {
            let a = r1.captures(lines[0]).unwrap();
            if r2.is_match(lines[1]) {
                let b = r2.captures(lines[1]).unwrap();
                if r3.is_match(lines[2]) {
                    let c = r3.captures(lines[2]).unwrap();
                    let case = Case{
                        Ax: a[1].parse::<i32>().unwrap(),
                        Ay: a[2].parse::<i32>().unwrap(),
                        Bx: b[1].parse::<i32>().unwrap(),
                        By: b[2].parse::<i32>().unwrap(),
                        Targetx: c[1].parse::<i32>().unwrap(),
                        Targety: c[2].parse::<i32>().unwrap(),
                    };
                    res.cases.push(case);
                }
            }   
        };
    };
    return res;
}

impl Case {
    fn metrics(&self) -> i32 {
        let mut cost = 0;
        for axi in 0..100 {
            let vAx = self.Ax * axi;
            if vAx >= self.Targetx {
                break;
            };
            let remx = self.Targetx - vAx;
            if remx % self.Bx != 0 {
                continue;
            }
            let bxi = remx / self.Bx;
            if bxi * self.By + axi * self.Ay == self.Targety {
                if cost == 0 {
                    cost = axi * 3 + bxi;
                } else {
                    cost = cost.min(axi * 3 + bxi);
                }
            }
        }
        return cost;
    }
}


impl Inp {
    fn metrics(&self) -> i32 {
        let mut res = 0;
        for c in &self.cases {
            res += c.metrics();
        }
       return res;
    }
    fn metrics2(&self) -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_works() {
        let c = Case{
            Ax: 94,
            Ay: 34,
            Bx: 22,
            By: 67,
            Targetx: 8400,
            Targety: 5400,
        };
        assert_eq!(c.metrics(), 280)
    }

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 480);
        //assert_eq!(read_file("test2.txt").metrics2(), 48);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
