use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse(line: &str) -> Pos {
    let parts: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    Pos {
        x: parts[0],
        y: parts[1],
    }
}

struct Field {
    walls: Vec<Vec<bool>>,
    width: i32,
    height: i32,
    distance: Vec<Vec<i32>>,
}

impl Field {
    fn put(&mut self, p: &Pos) {
        self.walls[p.y as usize][p.x as usize] = true;
    }
    fn nbrs(&self, p: &Pos) -> Vec<Pos> {
        let mut res = Vec::new();
        if p.x > 0 && !self.walls[p.y as usize][p.x as usize - 1] {
            res.push(Pos { x: p.x - 1, y: p.y });
        }
        if p.x < self.width - 1 && !self.walls[p.y as usize][p.x as usize + 1] {
            res.push(Pos { x: p.x + 1, y: p.y });
        }
        if p.y > 0 && !self.walls[p.y as usize - 1][p.x as usize] {
            res.push(Pos { x: p.x, y: p.y - 1 });
        }
        if p.y < self.height - 1 && !self.walls[p.y as usize + 1][p.x as usize] {
            res.push(Pos { x: p.x, y: p.y + 1 });
        }
        return res;
    }

    fn clean(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.distance[i as usize][j as usize] = -1;
            }
        }
    }

    fn wave(&mut self) {
        self.distance[0][0] = 0;
        let mut wave: HashMap<Pos, i32> = HashMap::new();
        wave.insert(Pos { x: 0, y: 0 }, 0);
        let mut newwave: HashMap<Pos, i32> = HashMap::new();
        for i in 1..self.height * self.width {
            //println!("wave: {:?}", wave);
            if wave.len() == 0 {
                break;
            }
            wave.iter().for_each(|p| {
                let nbrs = self.nbrs(p.0);
                for n in nbrs {
                    if self.distance[n.y as usize][n.x as usize] != -1 {
                        continue;
                    }
                    newwave.insert(n, i);
                }
            });
            newwave.iter().for_each(|p| {
                self.distance[p.0.y as usize][p.0.x as usize] = *p.1;
            });
            wave = newwave;
            newwave = HashMap::new();
        }
    }
}

fn empty_field(width: i32, height: i32) -> Field {
    let mut res = Field {
        walls: Vec::new(),
        width: width,
        height: height,
        distance: Vec::new(),
    };
    for _ in 0..height {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        for _ in 0..width {
            row.push(false);
            row2.push(-1 as i32);
        }
        res.walls.push(row);
        res.distance.push(row2);
    }
    return res;
}

#[derive(Debug)]
struct Inp {
    pos: Vec<Pos>,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp: Inp = Inp { pos: Vec::new() };
    for line in buf.lines() {
        inp.pos.push(parse(line));
    }
    return inp;
}

impl Inp {
    fn metrics(&self, field_size: i32, steps: i32) -> i32 {
        let mut field = empty_field(field_size, field_size);
        let mut s = 0;
        for p in &self.pos {
            if s >= steps {
                break;
            }
            s += 1;
            field.put(p);
        }
        field.wave();
        return field.distance[field_size as usize - 1][field_size as usize - 1];
    }
    fn metrics2(&self, field_size: i32) -> (i32, i32) {
        let mut field = empty_field(field_size, field_size);
        for p in &self.pos {
            field.put(p);
            field.clean();
            field.wave();
            if field.distance[field_size as usize - 1][field_size as usize - 1] == -1 {
                return (p.x, p.y);
            }
        }
        return (-1, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(7, 12), 22);
        assert_eq!(read_file("test.txt").metrics2(7), (6, 1));
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics(71, 1024));
    println!("Part2 {:?}", v.metrics2(71));
}
