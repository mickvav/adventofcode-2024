// use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Inp {
    m : Vec<Vec<char>>,
}

#[derive(Debug)]
struct Pos {
    x : i32,
    y : i32,
}

impl Pos {
    fn step(&mut self, dir :&Dir) {
        self.x += dir.x;
        self.y += dir.y;
    }
    fn next(&self, dir: &Dir) -> Pos {
        Pos {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

#[derive(Debug)]
struct Dir {
    x : i32,
    y : i32,
}

impl Dir {
    fn next(&mut self) {
        match self {
            Dir { x: 0, y: -1 } => *self = Dir { x: 1, y: 0},
            Dir { x: 1, y: 0 } => *self = Dir { x: 0, y: 1},
            Dir { x: 0, y: 1 } => *self = Dir { x: -1, y: 0},
            Dir { x: -1, y: 0 } => *self = Dir { x: 0, y: -1},
            _=> panic!("bad direction")
        }
    }
}




fn read_file(filename: &str) -> Inp {
    let mut inp = Inp {
        m: Vec::new(),
    };
    let mut lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in lines.lines() {
        inp.m.push(line.chars().collect());
    }
    return inp;
}

impl Inp {
    fn get_guard(&self) -> Pos {
        let mut posx : usize = 0;
        let mut posy : usize = 0;
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                if self.m[y][x] == '^' {
                    return Pos{x: x as i32, y: y as i32};
                }
            }
        }
        return Pos{x: posx as i32, y: posy as i32};
    }
    fn in_field(&self, p: &Pos) -> bool {
        return p.y >= 0 && p.y < self.m.len() as i32 && p.x >= 0 && p.x < self.m[p.y as usize].len() as i32;
    }
    fn is_wall(&self, p: &Pos) -> bool {
        return self.m[p.y as usize][p.x as usize] == '#';
    }
    fn metrics(&mut self) -> i32 {
        let mut pos = self.get_guard();
        let mut dir = Dir { x: 0, y: -1 };

        while self.in_field(&pos) {
            let n = pos.next(&dir);
            if self.in_field(&n) {
                if !self.is_wall(&n) {
                  self.m[pos.y as usize][pos.x as usize] = 'X';
                    self.m[n.y as usize][n.x as usize] = 'X';
                    pos = n;
                } else {
                    dir.next();
                }
            } else {
                pos.step(&dir);
            }
        };
        let mut res = 0;
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                if self.m[y][x] == 'X' {
                    res += 1;
                }
                print!("{}", self.m[y][x]);
            }
            println!();
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
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 41);
        assert_eq!(read_file("test.txt").metrics2(),0);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
