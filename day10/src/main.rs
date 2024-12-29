use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    topo: Vec<Vec<i8>>,
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp { topo: Vec::new() };
    let mut line = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for s in line.lines() {
        let mut v: Vec<i8> = Vec::new();
        for c in s.chars() {
            v.push(c as i8 - '0' as i8);
        }
        inp.topo.push(v);
    }
    return inp;
}

impl Inp {
    fn heads(&self, x: usize, y: usize, visited: &mut HashMap<(usize, usize), bool>) -> i8 {
        let h = self.topo[y][x];
        if h == 9 {
            if visited.contains_key(&(x, y)) {
                return 0;
            }
            visited.insert((x, y), true);
            return 1;
        };
        let mut res = 0;
        if x > 0 {
            if self.topo[y][x - 1] == h + 1 {
                res += self.heads(x - 1, y, visited);
            }
        }
        if x < self.topo[y].len() - 1 {
            if self.topo[y][x + 1] == h + 1 {
                res += self.heads(x + 1, y, visited);
            }
        }
        if y > 0 {
            if self.topo[y - 1][x] == h + 1 {
                res += self.heads(x, y - 1, visited);
            }
        }
        if y < self.topo.len() - 1 {
            if self.topo[y + 1][x] == h + 1 {
                res += self.heads(x, y + 1, visited);
            }
        }

        return res;
    }

    fn heads2(&self, x: usize, y: usize, visited: &mut HashMap<(usize, usize), bool>) -> i8 {
        let h = self.topo[y][x];
        if h == 9 {
            if visited.contains_key(&(x, y)) {
                return 1;
            }
            visited.insert((x, y), true);
            return 1;
        };
        let mut res = 0;
        if x > 0 {
            if self.topo[y][x - 1] == h + 1 {
                res += self.heads2(x - 1, y, visited);
            }
        }
        if x < self.topo[y].len() - 1 {
            if self.topo[y][x + 1] == h + 1 {
                res += self.heads2(x + 1, y, visited);
            }
        }
        if y > 0 {
            if self.topo[y - 1][x] == h + 1 {
                res += self.heads2(x, y - 1, visited);
            }
        }
        if y < self.topo.len() - 1 {
            if self.topo[y + 1][x] == h + 1 {
                res += self.heads2(x, y + 1, visited);
            }
        }

        return res;
    }

    fn metrics(&self) -> i32 {
        let mut res = 0;

        for y in 0..self.topo.len() {
            for x in 0..self.topo[y].len() {
                if self.topo[y][x] == 0 {
                    let mut visited = HashMap::new();
                    res += self.heads(x, y, &mut visited) as i32;
                }
            }
        }
        return res;
    }
    fn metrics2(&self) -> i32 {
        let mut res = 0;
        for y in 0..self.topo.len() {
            for x in 0..self.topo[y].len() {
                if self.topo[y][x] == 0 {
                    let mut visited = HashMap::new();
                    res += self.heads2(x, y, &mut visited) as i32;
                }
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
        assert_eq!(read_file("test.txt").metrics(), 36);
        assert_eq!(read_file("test2.txt").metrics2(), 0);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
