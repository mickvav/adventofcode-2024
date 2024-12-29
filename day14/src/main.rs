// use std::collections::HashMap;
use regex::Regex;
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct State {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}
#[derive(Debug)]
struct Inp {
    robots: Vec<State>,
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp { robots: Vec::new() };
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let re = Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    for (_, [x, y, vx, vy]) in re.captures_iter(&buf).map(|c| c.extract()) {
        inp.robots.push(State {
            x: x.parse::<i64>().unwrap(),
            y: y.parse::<i64>().unwrap(),
            vx: vx.parse::<i64>().unwrap(),
            vy: vy.parse::<i64>().unwrap(),
        });
    }
    println!("robots: {:?}", inp.robots);
    return inp;
}

impl Inp {
    fn metrics(&self, xsize: i64, ysize: i64) -> i32 {
        let mut res = 0;
        let mut predicted = Vec::new();
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        let x2 = xsize / 2;
        let y2 = ysize / 2;
        let steps: i64 = 100;
        for robot in &self.robots {
            // do 100 steps, wrap
            let mut vx = robot.vx;
            let mut vy = robot.vy;
            if vx < 0 {
                vx += xsize;
            };
            if vy < 0 {
                vy += ysize;
            };
            if vx > xsize {
                vx -= xsize;
            };
            if vy > ysize {
                vy -= ysize;
            };
            if vx < 0 || vy < 0 {
                panic!("{} {}", vx, vy);
            }
            let px = (robot.x + vx * steps) % xsize;
            let py = (robot.y + vy * steps) % ysize;
            if px < x2 && py < y2 {
                q1 += 1;
            }
            if px > x2 && py < y2 {
                q2 += 1;
            }
            if px > x2 && py > y2 {
                q3 += 1;
            }
            if px < x2 && py > y2 {
                q4 += 1;
            }
            predicted.push(State {
                x: px,
                y: py,
                vx: robot.vx,
                vy: robot.vy,
            });
        }
        println!("predicted: {:?}", predicted);
        let mut file = File::create("rustres.txt").unwrap();
        let mut i = 0;
        for p in predicted.iter() {
            let _ = file.write_all(format!("{},{},{}\n", i, p.x, p.y).as_bytes());
            i += 1;
        }
        file.flush().unwrap();

        println!("q1: {:?}", q1);
        println!("q2: {:?}", q2);
        println!("q3: {:?}", q3);
        println!("q4: {:?}", q4);
        println!("x2: {:?}", x2);
        println!("y2: {:?}", y2);
        println!("{}", self.robots.len());
        res = q1 * q2 * q3 * q4;
        return res;
    }
    fn metrics2(&self, xsize: i64, ysize: i64) -> i32 {
        let mut res = 0;
        let x2 = xsize / 2;
        let y2 = ysize / 2;
        let mut minm = 1000000000;
        let mut minqm = 1000000000;
        for res in 0..1000000 {
            let mut q1 = 0;
            let mut q2 = 0;
            let mut q3 = 0;
            let mut q4 = 0;
            let mut good = true;
            let mut m = 0;
            for robot in &self.robots {
                let mut vx = robot.vx;
                let mut vy = robot.vy;
                if vx < 0 {
                    vx += xsize;
                };
                if vy < 0 {
                    vy += ysize;
                };
                let px = (robot.x + vx * res) % xsize;
                let py = (robot.y + vy * res) % ysize;
                if py > min(px * 2, ysize * 2 - px * 2) {
                    good = false;
                    m += py - min(px * 2, ysize * 2 - px * 2);
                    //break;
                }
                if px < x2 && py < y2 {
                    q1 += 1;
                }
                if px > x2 && py < y2 {
                    q2 += 1;
                }
                if px < x2 && py > y2 {
                    q3 += 1;
                }
                if px > x2 && py > y2 {
                    q4 += 1;
                }
            }
            if m < minm {
                minm = m;
            }
            let qm = q1 * q2 * q3 * q4;
            if qm < minqm {
                minqm = qm;
            }
            println!("{} {} {} {:?}", minqm, minm, res, m);
            if minqm == 41306967 {
                break;
            }
            if good {
                println!("res: {:?}", res);
                return res.try_into().unwrap();
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
        assert_eq!(read_file("test.txt").metrics(11, 7), 13);
        assert_eq!(read_file("test2.txt").metrics2(11, 7), 0);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics(101, 103));
    println!("Part2 {:?}", v.metrics2(101, 103));
}
