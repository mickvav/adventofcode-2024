// use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Key {
    heights: [i32;5]
}

#[derive(Debug)]
struct Lock {
    heights: [i32;5]
}

impl Lock {
    fn fit(&self, key: &Key) -> bool {
        let mut res = true;
        for i in 0..5 {
            if self.heights[i] + key.heights[i] > 5 {
                res = false;
                break;
            }
        }
        return res
    }
    fn parse(lines: Vec<String>) -> Lock {
        let mut res = Lock {
            heights: [-1;5]
        };
        for line in lines {
            let mut i = 0;
            for c in line.chars() {
                if c == '#' {
                    res.heights[i] += 1;
                }
                i = i+1;
            }
        }
        return res;
    }
}

impl Key {
    fn parse(lines: Vec<String>) -> Key {
        let mut res = Key {
            heights: [-1;5]
        };
        for line in lines.iter().rev() {
            let mut i = 0;
            for c in line.chars() {
                if c == '#' {
                    res.heights[i] += 1;
                }
                i = i+1;
            }
        }
        return res;
    }
}
#[derive(Debug)]
struct Inp {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

fn read_file(filename: &str) -> Inp {
    let mut lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp { 
        locks: Vec::new(),
        keys: Vec::new()
    };
    let mut buf: Vec<String> = Vec::new();
    for line in lines.lines() {
        if line.len() == 0 {
            if buf.len() > 0 {
                if buf[0] == "#####".to_string() {
                    inp.locks.push(Lock::parse(buf.clone()));
                } else {
                    inp.keys.push(Key::parse(buf.clone()));
                }
                buf.clear();
            }
        } else {
            buf.push(line.to_string());
        }
    };
    return inp;
}

impl Inp {
    fn metrics(&self) -> i32 {
        let mut res = 0;
        for l in self.locks.iter() {
            println!("lock: {:?}", l);
            println!("keys:");
            for k in self.keys.iter() {
                println!("{:?}", k);
                if l.fit(k) {
                    res += 1;
                }
            }
        }
       return res;
    }
    fn metrics2(&self) -> i32 {
        let mut res = 0;

       return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 3);
        assert_eq!(read_file("test2.txt").metrics2(), 0);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
