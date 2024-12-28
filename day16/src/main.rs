use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    x: u8,
    y: u8,
    direction: i16, // just for alignment
                    //   ^ - 0, > - 1, v - 2, < - 3
}

impl State {
    fn strip(&self) -> State {
        State {
            x: self.x,
            y: self.y,
            direction: 0,
        }
    }
}

#[derive(Debug)]
struct Inp {
    lines: Vec<String>,
    target: State,
}

#[derive(Debug)]
struct Wave {
    states: HashMap<State, i32>,
    time: i32,
}

impl Wave {
    fn new() -> Wave {
        Wave {
            states: HashMap::new(),
            time: 0,
        }
    }

    fn init(inp: &Inp) -> Wave {
        let mut res = Wave::new();
        for y in 0..inp.lines.len() {
            for x in 0..inp.lines[y].len() {
                if inp.lines[y].chars().nth(x).unwrap() == 'S' {
                    res.states.insert(
                        State {
                            x: x as u8,
                            y: y as u8,
                            direction: 1,
                        },
                        0,
                    );
                }
            }
        }
        return res;
    }

    fn next(&self, inp: &Inp) -> (Wave, bool) {
        let mut res = Wave::new();
        res.time = self.time + 1;
        let mut can_continue = false;
        self.states.iter().for_each(|(s, t)| {
            res.states.insert(s.clone(), *t);
        });
        self.states.iter().for_each(|(s, t)| {
            let nbrs = inp.nbrs(s);
            nbrs.iter().for_each(|n| {
                let mut cost = 1;
                if n.direction != s.direction {
                    cost = 1000;
                }
                if self.states.contains_key(n) {
                    if self.states[n] > t + cost {
                        res.states.insert(n.clone(), t + cost);
                        can_continue = true;
                    } else {
                        res.states.insert(n.clone(), self.states[n]);
                    }
                } else {
                    res.states.insert(n.clone(), t + cost);
                    can_continue = true;
                }
            });
        });
        return (res, can_continue);
    }

    fn walk_back(&self, x: u8, y: u8, d: i16, inp: &Inp) -> i32 {
        let mut spots: HashMap<State, bool> = HashMap::new();
        let mut res = 0;
        let mut can_continue = true;
        let mut backwave: Vec<State> = Vec::new();
        let mut nextbackwave: HashMap<State, bool> = HashMap::new();
        backwave.push(State {
            x: x,
            y: y,
            direction: d,
        });
        while can_continue {
            can_continue = false;
            nextbackwave.clear();
            println!("{:?}", backwave);
            for s in &backwave {
                let nbrs = inp.nbrs_rev(&s);
                let v = self.states[&s];
                nbrs.iter().for_each(|n| {
                    let nv = self.states[n];
                    //  println!("v: {}, nv: {}, n: {:?}", v, nv, n);
                    if n.direction == s.direction && v - nv == 1 {
                        //  println!("+");
                        nextbackwave.insert(n.clone(), true);
                        spots.insert(n.strip(), true);
                        can_continue = true;
                    } else {
                        if v - nv == 1000 {
                            println!("+");
                            nextbackwave.insert(n.clone(), true);
                            spots.insert(n.strip(), true);
                            can_continue = true;
                        }
                    }
                });
            }
            backwave.clear();
            for s in nextbackwave.keys() {
                backwave.push(s.clone());
            }
        }
        return spots.len() as i32;
    }
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp {
        lines: Vec::new(),
        target: State {
            x: 0,
            y: 0,
            direction: 0,
        },
    };
    let mut y = 0;
    for line in lines.lines() {
        inp.lines.push(line.to_string());
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == 'E' {
                inp.target = State {
                    x: x as u8,
                    y: y as u8,
                    direction: 0,
                };
            }
        }
        y += 1;
    }
    return inp;
}

impl Inp {
    fn is_free(&self, x: u8, y: u8) -> bool {
        if y as usize >= self.lines.len() {
            return false;
        }
        let row = &self.lines[y as usize];
        if x as usize >= row.len() {
            return false;
        }
        return row.chars().nth(x as usize).unwrap() != '#';
    }

    fn nbrs_rev(&self, s: &State) -> Vec<State> {
        let mut res = Vec::new();
        if s.y > 0 && s.direction == 2 && self.is_free(s.x, s.y - 1) {
            res.push(State {
                x: s.x,
                y: s.y - 1,
                direction: 2,
            });
        }
        if s.x > 0 && s.direction == 1 && self.is_free(s.x - 1, s.y) {
            res.push(State {
                x: s.x - 1,
                y: s.y,
                direction: 1,
            });
        }
        if s.direction == 0 && self.is_free(s.x, s.y + 1) {
            res.push(State {
                x: s.x,
                y: s.y + 1,
                direction: 0,
            });
        }
        if s.direction == 3 && self.is_free(s.x + 1, s.y) {
            res.push(State {
                x: s.x + 1,
                y: s.y,
                direction: 3,
            });
        }
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 1) % 4,
        });
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 2) % 4,
        });
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 3) % 4,
        });
        return res;
    }
    fn nbrs(&self, s: &State) -> Vec<State> {
        let mut res = Vec::new();
        if s.y > 0 && s.direction == 0 && self.is_free(s.x, s.y - 1) {
            res.push(State {
                x: s.x,
                y: s.y - 1,
                direction: 0,
            });
        }
        if s.x > 0 && s.direction == 3 && self.is_free(s.x - 1, s.y) {
            res.push(State {
                x: s.x - 1,
                y: s.y,
                direction: 3,
            });
        }
        if s.direction == 2 && self.is_free(s.x, s.y + 1) {
            res.push(State {
                x: s.x,
                y: s.y + 1,
                direction: 2,
            });
        }
        if s.direction == 1 && self.is_free(s.x + 1, s.y) {
            res.push(State {
                x: s.x + 1,
                y: s.y,
                direction: 1,
            });
        }
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 1) % 4,
        });
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 2) % 4,
        });
        res.push(State {
            x: s.x,
            y: s.y,
            direction: (s.direction + 3) % 4,
        });
        return res;
    }
    fn metrics(&self) -> i32 {
        let mut res = i32::MAX;
        let mut w = Wave::init(self);
        let mut can_continue = true;
        while can_continue {
            // println!("{:?}", w);
            (w, can_continue) = w.next(self);
            println!("{}", w.states.len());
        }
        for i in 0..4 {
            if w.states.contains_key(&State {
                x: self.target.x,
                y: self.target.y,
                direction: i,
            }) {
                res = min(
                    w.states[&State {
                        x: self.target.x,
                        y: self.target.y,
                        direction: i,
                    }],
                    res,
                );
            }
        }
        println!(
            "wb: {:?}",
            w.walk_back(self.target.x, self.target.y, self.target.direction, self) + 1
        );
        return res;
    }
    fn metrics2(&self) -> i32 {
        let res = 0;

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 7036);
        assert_eq!(read_file("test2.txt").metrics2(), 0);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
