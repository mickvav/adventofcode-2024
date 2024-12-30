use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    m: Vec<Vec<char>>,
    g: Pos,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn step(&mut self, dir: &Dir) {
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Dir {
    x: i32,
    y: i32,
}

impl Dir {
    fn next(&mut self) {
        match self {
            Dir { x: 0, y: -1 } => *self = Dir { x: 1, y: 0 },
            Dir { x: 1, y: 0 } => *self = Dir { x: 0, y: 1 },
            Dir { x: 0, y: 1 } => *self = Dir { x: -1, y: 0 },
            Dir { x: -1, y: 0 } => *self = Dir { x: 0, y: -1 },
            _ => panic!("bad direction"),
        }
    }
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp {
        m: Vec::new(),
        g: Pos { x: 0, y: 0 },
    };
    let mut lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in lines.lines() {
        inp.m.push(line.chars().collect());
    }
    inp.g = inp.get_guard();
    return inp;
}

impl Inp {
    fn get_guard(&self) -> Pos {
        let mut posx: usize = 0;
        let mut posy: usize = 0;
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                if self.m[y][x] == '^' {
                    return Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                }
            }
        }
        return Pos {
            x: posx as i32,
            y: posy as i32,
        };
    }
    fn in_field(&self, p: &Pos) -> bool {
        return p.y >= 0
            && p.y < self.m.len() as i32
            && p.x >= 0
            && p.x < self.m[p.y as usize].len() as i32;
    }
    fn is_wall(&self, p: &Pos) -> bool {
        return self.m[p.y as usize][p.x as usize] == '#';
    }
    fn metrics(&mut self) -> i32 {
        let mut pos = self.g;
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
        }
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

    fn display(&self, hm: &HashMap<(Pos, Dir), bool>) {
        let basicdirs: [Dir; 4] = [
            Dir { x: 0, y: -1 },
            Dir { x: 1, y: 0 },
            Dir { x: 0, y: 1 },
            Dir { x: -1, y: 0 },
        ];
        println!("");
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                if self.is_wall(&Pos {
                    x: x as i32,
                    y: y as i32,
                }) {
                    print!("#");
                } else {
                    let pos = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                    let mut c = '.';
                    for d in basicdirs {
                        if hm.contains_key(&(pos, d)) {
                            c = 'X';
                        }
                    }
                    print!("{}", c);
                }
            }
            println!();
        }
    }
    fn is_loop(&self, start: &Pos) -> bool {
        let mut pos: Pos = *start;
        let mut dir = Dir { x: 0, y: -1 };

        let mut hm: HashMap<(Pos, Dir), bool> = HashMap::new();
        while self.in_field(&pos) {
            if hm.contains_key(&(pos.clone(), dir.clone())) {
                return true;
            }
            hm.insert((pos.clone(), dir.clone()), true);
            let n = pos.next(&dir);
            if self.in_field(&n) {
                if !self.is_wall(&n) {
                    pos = n;
                } else {
                    dir.next();
                }
            } else {
                pos.step(&dir);
            }
        }
        //self.display(&hm);
        return false;
    }
    fn cleanup(&mut self) -> Vec<Pos> {
        let mut res: Vec<Pos> = Vec::new();
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                if self.m[y][x] == 'X' {
                    self.m[y][x] = '.';
                    res.push(Pos {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        return res;
    }
    fn metrics2(&mut self) -> i32 {
        let mut res = 0;
        let candidates = self.cleanup();
        for cp in candidates {
            if cp == self.g {
                continue;
            }
            let y = cp.y;
            let x = cp.x;
            self.m[y as usize][x as usize] = '#';
            if self.is_loop(&self.g) {
                res += 1;
            }
            self.m[y as usize][x as usize] = '.';
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 41);
        assert_eq!(read_file("test.txt").metrics2(), 6);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
