// use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Case {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    targetx: i64,
    targety: i64,
}
#[derive(Debug)]
struct Inp {
    cases: Vec<Case>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp { cases: vec![] };
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
                    let case = Case {
                        ax: a[1].parse::<i64>().unwrap(),
                        ay: a[2].parse::<i64>().unwrap(),
                        bx: b[1].parse::<i64>().unwrap(),
                        by: b[2].parse::<i64>().unwrap(),
                        targetx: c[1].parse::<i64>().unwrap(),
                        targety: c[2].parse::<i64>().unwrap(),
                    };
                    res.cases.push(case);
                }
            }
        };
    }
    return res;
}

impl Case {
    fn metrics(&self) -> i64 {
        let mut cost = 0;
        for axi in 0..100 {
            let v_ax = self.ax * axi;
            if v_ax >= self.targetx {
                break;
            };
            let remx = self.targetx - v_ax;
            if remx % self.bx != 0 {
                continue;
            }
            let bxi = remx / self.bx;
            if bxi * self.by + axi * self.ay == self.targety {
                if cost == 0 {
                    cost = axi * 3 + bxi;
                } else {
                    cost = cost.min(axi * 3 + bxi);
                }
            }
        }
        return cost;
    }
    fn metrics2(&self) -> i64 {
        let base: i64 = 10000000000000;

        // Find intersection of lines:
        // (ax*i, ay *i)
        // ((base + targetx) - bx*j, (base + targety) - by*j)
        // if whole coords, use them
        // ax*i = (base + targetx) - bx*j
        // ay*i = (base + targety) - by*j
        // ((ax/bx) - (ay/by)) * i = (base + targetx)/bx - (base + targety)/by
        // i =((base + targetx) * by - (base + targety) * bx)/(ax*by - ay*bx)
        // 0 = (base + targetx) / ax - (base + targety) / ay - (bx/ax - by/ay ) *j
        // j = ((base + targetx) * ay - (base + targety) * ax) / (bx*ay - by*ax)
        // j = ( (base + targety) * ax - (base + targetx) * ay) / det
        let det = self.ax * self.by - self.ay * self.bx;
        let det_b = self.bx * (self.targetx + base) - self.by * (self.targety + base);
        let det_a = self.ax * (self.targety + base) - self.ay * (self.targetx + base);
        if det == 0 {
            // Lines are ||, so if b hits target alone - it is the solution
            // otherwise smallest number of a that allows to hit the target gives the solutio
            if det_b != 0 {
                return 0;
            }
            let ib0 = (base + self.targetx) / self.bx + 1;
            let mut seen = HashSet::new();
            for ib in (0..ib0).rev() {
                let rem = (base + self.targetx) - self.bx * ib;
                if rem % self.ax == 0 {
                    let ia = rem / self.ax;
                    return (3 * ia + ib) as i64;
                }
                let modulo = rem % self.ax;
                if modulo < 0 {
                    continue;
                }
                if seen.contains(&modulo) {
                    return 0;
                }
                seen.insert(modulo);
            }
        } else {
            if det_a == 0 {
                // A || target
                let rem = (base + self.targetx) % self.ax;
                if rem != 0 {
                    return 0;
                }
                let i = (base + self.targetx) / self.ax;
                return 3 * i as i64;
            }
            if det_b == 0 {
                // B || target
                let rem = (base + self.targety) % self.by;
                if rem != 0 {
                    return 0;
                }
                let i = (base + self.targety) / self.by;
                return i.into();
            }
            // All non-0
            let i = ((base + self.targetx) * self.by - (base + self.targety) * self.bx) / det;
            if i < 0 {
                return 0;
            }
            let j = ((base + self.targety) * self.ax - (base + self.targetx) * self.ay) / det;
            if j < 0 {
                return 0;
            }
            let landedx = i * self.ax + j * self.bx;
            if landedx == self.targetx + base {
                let landedy = i * self.ay + j * self.by;
                if landedy == self.targety + base {
                    return (3 * i + j).into();
                } else {
                    return 0;
                }
            } else {
                return 0;
            };
        }
        return 0;
    }
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut res = 0;
        for c in &self.cases {
            res += c.metrics();
        }
        return res;
    }
    fn metrics2(&self) -> i64 {
        let mut res = 0;
        for c in &self.cases {
            res += c.metrics2();
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_works() {
        let c = Case {
            ax: 94,
            ay: 34,
            bx: 22,
            by: 67,
            targetx: 8400,
            targety: 5400,
        };
        assert_eq!(c.metrics(), 280)
    }

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 480);
        assert_eq!(read_file("test.txt").metrics2(), 875318608908);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
