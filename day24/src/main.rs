use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

type WireName = [char; 3];

fn wire_name_from_string(s: &str) -> WireName {
    let mut res = ['\0'; 3];
    for i in 0..3 {
        res[i] = s.chars().nth(i).unwrap();
    }
    return res;
}

#[derive(Debug, Clone)]
struct Wire {
    name: WireName,
    gatestate: GateState,
}

#[derive(Debug, PartialEq, Clone)]
enum GateType {
    AND,
    OR,
    XOR,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GateState {
    ON,
    OFF,
    UNDECIDED,
}

#[derive(Debug, Clone)]
struct Gate {
    t: GateType,
    a: WireName,
    b: WireName,
    out: WireName,
}

#[derive(Debug, Clone)]
struct State {
    wires: HashMap<WireName, Wire>,
    gates: Vec<Gate>,
    steps: i64,
}

impl State {
    fn next(&self) -> State {
        let mut res = self.clone();
        res.steps = 0;
        for g in self.gates.iter() {
            let s1 = self.wires[&g.a].gatestate;
            let s2 = self.wires[&g.b].gatestate;
            let s3 = self.wires[&g.out].gatestate;
            if s1 == GateState::UNDECIDED {
                continue;
            };
            if s2 == GateState::UNDECIDED {
                continue;
            };
            if s3 != GateState::UNDECIDED {
                continue;
            };
            res.steps += 1;
            if g.t == GateType::AND {
                if s1 == GateState::ON && s2 == GateState::ON {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::ON);
                } else {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::OFF);
                }
            }
            if g.t == GateType::OR {
                if s1 == GateState::ON || s2 == GateState::ON {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::ON);
                } else {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::OFF);
                }
            }
            if g.t == GateType::XOR {
                if s1 != s2 {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::ON);
                } else {
                    res.wires
                        .entry(g.out)
                        .and_modify(|w| w.gatestate = GateState::OFF);
                }
            }
        }
        return res;
    }

    fn summator_init(&self, x: i64, y: i64) -> State {
        let mut res = self.clone();
        for w in res.wires.values_mut() {
            w.gatestate = GateState::UNDECIDED;
        }

        for i in 0..44 {
            let xk: WireName = [
                'x',
                (i / 10).to_string().chars().nth(0).unwrap(),
                (i % 10).to_string().chars().nth(0).unwrap(),
            ];
            if (x >> i) & 0x1 == 1 {
                res.wires
                    .entry(xk)
                    .and_modify(|w| w.gatestate = GateState::ON);
            } else {
                res.wires
                    .entry(xk)
                    .and_modify(|w| w.gatestate = GateState::OFF);
            }
            let yk: WireName = [
                'y',
                (i / 10).to_string().chars().nth(0).unwrap(),
                (i % 10).to_string().chars().nth(0).unwrap(),
            ];
            if (y >> i) & 0x1 == 1 {
                res.wires
                    .entry(yk)
                    .and_modify(|w| w.gatestate = GateState::ON);
            } else {
                res.wires
                    .entry(yk)
                    .and_modify(|w| w.gatestate = GateState::OFF);
            }
        }
        return res;
    }

    fn is_finished(&self) -> bool {
        if self.steps == 0 {
            return true;
        }
        for w in self.wires.values() {
            //println!("{:?} {:?}", w, w.gatestate);
            if w.name[0] == 'z' && w.gatestate == GateState::UNDECIDED {
                return false;
            }
        }
        return true;
    }

    fn get_value(&self) -> i64 {
        let mut names: Vec<String> = Vec::new();
        for w in self.wires.values() {
            if w.name[0] == 'z' {
                names.push(String::from_iter(w.name));
            }
        }
        names.sort();
        let mut res: i64 = 0;
        for n in names {
            let p = n[1..].to_string().parse::<i32>().unwrap();
            let wn = wire_name_from_string(&n);
            if self.wires[&wn].gatestate == GateState::ON {
                res |= 1 << p;
            }
        }
        return res;
    }

    fn get_string(&self) -> String {
        let mut s = "".to_string();
        let mut names: Vec<String> = Vec::new();
        for w in self.wires.values() {
            if w.name[0] == 'z' {
                names.push(String::from_iter(w.name));
            }
        }
        names.sort();
        names.reverse();
        for n in names {
            let p = n[1..].to_string().parse::<i32>().unwrap();
            let wn = wire_name_from_string(&n);
            if self.wires[&wn].gatestate == GateState::ON {
                s += "1";
            }
            if self.wires[&wn].gatestate == GateState::OFF {
                s += "0";
            }
            if self.wires[&wn].gatestate == GateState::UNDECIDED {
                s += "X";
            }
        }
        return s;
    }
}
#[derive(Debug)]
struct Inp {
    state: State,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = buf.lines();
    let mut res = Inp {
        state: State {
            wires: HashMap::new(),
            gates: Vec::new(),
            steps: 0,

        },
    };

    let mut reading_gates = false;
    for line in lines {
        if !reading_gates {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                reading_gates = true;
                continue;
            }
            let wn = wire_name_from_string(parts[0]);
            let mut ws = GateState::UNDECIDED;
            if parts[1] == "1" {
                ws = GateState::ON;
            } else if parts[1] == "0" {
                ws = GateState::OFF;
            }
            res.state.wires.insert(
                wn,
                Wire {
                    name: wn,
                    gatestate: ws,
                },
            );
        } else {
            let parts: Vec<&str> = line.split(" -> ").collect();
            if parts.len() != 2 {
                continue;
            };
            let eqprts: Vec<&str> = parts[0].split(" ").collect();
            if eqprts.len() != 3 {
                continue;
            };
            let wn1 = wire_name_from_string(eqprts[0]);
            let wn2 = wire_name_from_string(eqprts[2]);
            let wn3 = wire_name_from_string(parts[1]);
            if !res.state.wires.contains_key(&wn1) {
                res.state.wires.insert(
                    wn1,
                    Wire {
                        name: wn1,
                        gatestate: GateState::UNDECIDED,
                    },
                );
            }
            if !res.state.wires.contains_key(&wn2) {
                res.state.wires.insert(
                    wn2,
                    Wire {
                        name: wn2,
                        gatestate: GateState::UNDECIDED,
                    },
                );
            }
            let mut gt = GateType::AND;
            if eqprts[1] == "OR" {
                gt = GateType::OR;
            } else if eqprts[1] == "XOR" {
                gt = GateType::XOR;
            }
            res.state.gates.push(Gate {
                t: gt,
                a: wn1,
                b: wn2,
                out: wn3,
            });
            if !res.state.wires.contains_key(&wn3) {
                res.state.wires.insert(
                    wn3,
                    Wire {
                        name: wn3,
                        gatestate: GateState::UNDECIDED,
                    },
                );
            }
        }
    }
    //println!("{:?}", res);
    return res;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut s = self.state.clone();
        let start = Instant::now();
        s.steps = 1;
        while !s.is_finished() {
            //println!("===step===");
            //println!("{:?}", s);
            s = s.next();
        }
        let duration = start.elapsed();
        println!("{:?}", duration);

        return s.get_value();
    }
    fn metrics2(&self) -> i64 {
        let mut s = self.state.clone();
        let start = Instant::now();

        for i in 0..45 {
            s = s.summator_init(0,1 << i as i64);
            s.steps = 1;
            while !s.is_finished() {
                s = s.next();
                //println!(".");
            }
            println!("{} {}", i, s.get_string());
            let v = s.get_value();
            if v != 1 << i as i64 {
                println!("==== {} {}", i, v)
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").state.is_finished(), false);
        assert_eq!(read_file("test.txt").metrics(), 2024);
        assert_eq!(read_file("test.txt").metrics2(), 0)
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    let v1 = read_file("input1.txt");
    println!("Part2 {:?}", v1.metrics2());
}
