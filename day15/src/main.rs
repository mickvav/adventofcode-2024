use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    fn to(&self, dir: char) -> Pos {
        match dir {
            '^' => {
                return Pos {
                    x: self.x,
                    y: self.y - 1,
                };
            }
            '>' => {
                return Pos {
                    x: self.x + 1,
                    y: self.y,
                };
            }
            '<' => {
                return Pos {
                    x: self.x - 1,
                    y: self.y,
                }
            }
            'v' => {
                return Pos {
                    x: self.x,
                    y: self.y + 1,
                }
            }
            _ => {
                return self.clone();
            }
        }
    }
}

#[derive(Debug)]
struct Inp {
    lines: Vec<Vec<char>>,
    lines2: Vec<Vec<char>>,
    program: String,
    robot: Pos,
    robot2: Pos,
}
fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp {
        lines: Vec::new(),
        lines2: Vec::new(),
        robot: Pos { x: 0, y: 0 },
        robot2: Pos { x: 0, y: 0 },
        program: String::new(),
    };
    let mut y = 0;
    let mut reading_program = false;
    for line in lines.lines() {
        if line.len() == 0 {
            reading_program = true;
            continue;
        }
        if reading_program {
            inp.program.push_str(line);
            continue;
        }
        inp.lines.push(Vec::new());
        inp.lines2.push(Vec::new());
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '@' {
                inp.robot = Pos {
                    x: x as u16,
                    y: y as u16,
                };
                inp.robot2 = Pos {
                    x: (x * 2) as u16,
                    y: y as u16,
                };
                inp.lines[y].push('.');
                inp.lines2[y].push('.');
                inp.lines2[y].push('.');
            } else {
                inp.lines[y].push(line.chars().nth(x).unwrap());
                if line.chars().nth(x).unwrap() == 'O' {
                    inp.lines2[y].push('[');
                    inp.lines2[y].push(']');
                } else {
                    inp.lines2[y].push(line.chars().nth(x).unwrap());
                    inp.lines2[y].push(line.chars().nth(x).unwrap());
                }
            }
        }
        y += 1;
    }
    return inp;
}

impl Inp {
    fn is_free(&self, x: u16, y: u16) -> bool {
        return self.lines[y as usize][x as usize] != '#';
    }
    fn is_free2(&self, x: u16, y: u16) -> bool {
        return self.lines2[y as usize][x as usize] != '#';
    }

    fn string(&self) -> String {
        let mut res = String::new();
        for y in 0..self.lines.len() {
            for x in 0..self.lines[y].len() {
                if self.robot.x == x as u16 && self.robot.y == y as u16 {
                    res.push('@');
                } else {
                    res.push(self.lines[y][x]);
                }
            }
            res.push('\n');
        }
        return res;
    }

    fn string2(&self) -> String {
        let mut res = String::new();
        for y in 0..self.lines2.len() {
            for x in 0..self.lines2[y].len() {
                if self.robot2.x == x as u16 && self.robot2.y == y as u16 {
                    res.push('@');
                } else {
                    res.push(self.lines2[y][x]);
                }
            }
            res.push('\n');
        }
        return res;
    }

    fn can_move(&mut self, from: Pos, dir: char) -> (Pos, bool) {
        let mut to = from.to(dir);
        if !self.is_free(to.x, to.y) {
            return (from.clone(), false);
        }
        while self.lines[to.y as usize][to.x as usize] == 'O' {
            to = to.to(dir);
            if !self.is_free(to.x, to.y) {
                return (from, false);
            }
        }
        return (to, true);
    }
    // returns list of positions that should be moved and a flag
    fn can_move2(&mut self, from: Pos, dir: char) -> (Vec<Pos>, bool) {
        let mut to = from.to(dir);
        if !self.is_free2(to.x, to.y) {
            return (Vec::new(), false);
        };
        if dir == '<' || dir == '>' {
            let mut res = Vec::new();
            while self.lines2[to.y as usize][to.x as usize] == '['
                || self.lines2[to.y as usize][to.x as usize] == ']'
            {
                res.push(to.clone());
                to = to.to(dir);
                if !self.is_free2(to.x, to.y) {
                    return (Vec::new(), false);
                }
            }
            return (res, true);
        } else {
            let mut xv: Vec<u16> = Vec::new();
            xv.push(to.x);
            let mut y = to.y;
            let dy: i16 = if dir == '^' { -1 } else { 1 };
            let mut reshm: HashMap<Pos, bool> = HashMap::new();
            while xv.len() > 0 {
                let mut newxv: HashMap<u16, bool> = HashMap::new();
                for x in xv.iter() {
                    if !self.is_free2(*x, y) {
                        return (Vec::new(), false);
                    };
                    reshm.insert(Pos { x: *x, y: y }, true);
                    let c = self.lines2[y as usize][*x as usize];
                    if c == '[' {
                        newxv.insert(*x, true);
                        newxv.insert(x + 1, true);
                        reshm.insert(Pos { x: *x + 1, y: y }, true);
                    };
                    if c == ']' {
                        newxv.insert(*x, true);
                        newxv.insert(x - 1, true);
                        reshm.insert(Pos { x: *x - 1, y: y }, true);
                    };
                }
                y = (y as i16 + dy) as u16;
                xv = newxv.keys().cloned().collect();
                //  println!("{:?} , {}", xv, y);
                newxv.clear();
            }
            // println!("{:?}", reshm);
            return (reshm.keys().cloned().collect(), true);
        }
    }

    fn do_move2(&mut self, dir: char) {
        let (moved_boxes, ok) = self.can_move2(self.robot2.clone(), dir);
        if !ok {
            return;
        }
        let mut moved_hm: HashMap<Pos, char> = HashMap::new();
        for box_pos in moved_boxes.iter() {
            moved_hm.insert(
                box_pos.clone(),
                self.lines2[box_pos.y as usize][box_pos.x as usize],
            );
            self.lines2[box_pos.y as usize][box_pos.x as usize] = '.';
        }
        for box_pos in moved_boxes.iter() {
            let to = box_pos.to(dir);
            let c = moved_hm.get(box_pos).unwrap();
            if *c != '.' {
                self.lines2[to.y as usize][to.x as usize] = moved_hm.get(box_pos).unwrap().clone();
            }
        }
        self.robot2 = self.robot2.to(dir);
    }
    fn do_move(&mut self, dir: char) {
        let (to, ok) = self.can_move(self.robot.clone(), dir);
        println!("{}", dir);
        if !ok {
            return;
        }
        let mut moving_boxes = false;
        if to != self.robot.to(dir) {
            moving_boxes = true;
        }
        self.robot = self.robot.to(dir);
        println!("{:?} {:?} {}", self.robot, to, moving_boxes);
        if moving_boxes {
            self.lines[self.robot.y as usize][self.robot.x as usize] = '.';
            self.lines[to.y as usize][to.x as usize] = 'O';
        };
    }

    fn metrics(&mut self) -> i32 {
        let mut res = 0;
        let program = self.program.clone();
        for c in program.chars() {
            self.do_move(c);
            //  println!("{}", self.string());
            println!("");
        }
        println!("{}", self.string());
        for y in 0..self.lines.len() {
            for x in 0..self.lines[y].len() {
                if self.lines[y][x] == 'O' {
                    res += (y * 100 + x) as i32;
                }
            }
        }
        return res;
    }
    fn metrics2(&mut self) -> i32 {
        let mut res = 0;
        let program = self.program.clone();
        println!("{:?}", self.robot2);
        for c in program.chars() {
            self.do_move2(c);
            // println!("{}", self.string2());
        }
        for y in 0..self.lines2.len() {
            for x in 0..self.lines2[y].len() {
                if self.lines2[y][x] == '[' {
                    res += (y * 100 + x) as i32;
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
        assert_eq!(read_file("test1.txt").metrics(), 2028);
        assert_eq!(read_file("test.txt").metrics(), 10092);
        assert_eq!(read_file("test2.txt").metrics2(), 9021);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
