use rand::thread_rng;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn nbrs(&self) -> [Pos; 4] {
        return [
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
        ];
    }
    fn allnbrs(&self) -> [Pos; 8] {
        return [
            Pos {
                x: self.x - 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y + 1,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y + 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
        ];
    }
}

#[derive(Debug, Clone)]
struct Region {
    letter: char,
    p: HashMap<Pos, bool>,
}

impl Region {
    fn string(&self) -> String {
        let mut res = String::new();
        let mut minx = i32::MAX;
        let mut maxx = i32::MIN;
        let mut miny = i32::MAX;
        let mut maxy = i32::MIN;
        for p in self.p.keys() {
            if p.x < minx {
                minx = p.x;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.y > maxy {
                maxy = p.y;
            }
        }
        let mut lines = vec![String::new(); (maxy - miny + 1) as usize];
        for y in miny..maxy + 1 {
            for x in minx..maxx + 1 {
                if self.p.contains_key(&Pos { x, y }) {
                    lines[(y - miny) as usize].push(self.letter);
                } else {
                    lines[(y - miny) as usize].push('.');
                }
            }
        }
        for l in lines {
            res.push_str(&l);
            res.push('\n');
        }
        res.push_str(format!("x: {} {} y: {} {}", minx, maxx, miny, maxy).as_str());
        return res;
    }
    fn area(&self) -> i32 {
        return self.p.len() as i32;
    }
    fn nbrsmap(&self, p: &Pos) -> u8 {
        let mut res = 0;
        let mut i = 0;
        for n in p.allnbrs() {
            // println!("n:{:?}", n);
            if self.p.contains_key(&n) {
                res |= 1 << i;
            }
            i += 1;
        }
        return res;
    }

    fn perimeter(&self) -> i32 {
        let mut res = 0;
        for s in &self.p {
            for n in s.0.nbrs() {
                if !self.p.contains_key(&n) {
                    res += 1;
                }
            }
        }
        return res;
    }
    fn cost(&self) -> i32 {
        return self.area() * self.perimeter();
    }
}

#[derive(Debug)]
struct Inp {
    lines: Vec<String>,
    visited: Vec<Vec<bool>>,
    nbrs_to_corners: HashMap<u8, i8>,
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp {
        lines: Vec::new(),
        visited: Vec::new(),
        nbrs_to_corners: HashMap::new(),
    };

    let line = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for l in line.lines() {
        inp.lines.push(l.to_string());
        inp.visited.push(vec![false; l.len()]);
    }
    //  0 1 2
    //  7   3
    //  6 5 4
    inp.nbrs_to_corners.insert(0b00000000, 3 * 4);
    inp.nbrs_to_corners.insert(0b00000001, 3 * 4);
    inp.nbrs_to_corners.insert(0b00000010, 3 * 2);
    inp.nbrs_to_corners.insert(0b00000011, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00000100, 3 * 4);
    inp.nbrs_to_corners.insert(0b00000101, 3 * 4);
    inp.nbrs_to_corners.insert(0b00000110, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00000111, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b00001000, 3 * 2);
    inp.nbrs_to_corners.insert(0b00001001, 3 * 2);
    inp.nbrs_to_corners.insert(0b00001010, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00001011, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b00001100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00001101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00001110, 3);
    inp.nbrs_to_corners.insert(0b00001111, 3 + 1);
    inp.nbrs_to_corners.insert(0b00010000, 3 * 4);
    inp.nbrs_to_corners.insert(0b00010001, 3 * 4);
    inp.nbrs_to_corners.insert(0b00010010, 3 * 2);
    inp.nbrs_to_corners.insert(0b00010011, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00010100, 3 * 4);
    inp.nbrs_to_corners.insert(0b00010101, 3 * 4);
    inp.nbrs_to_corners.insert(0b00010110, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00010111, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b00011000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00011001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00011010, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b00011011, 3 * 1 + 3);
    inp.nbrs_to_corners.insert(0b00011100, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b00011101, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b00011110, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00011111, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b00100000, 3 * 2);
    inp.nbrs_to_corners.insert(0b00100001, 3 * 2);
    inp.nbrs_to_corners.insert(0b00100010, 0);
    inp.nbrs_to_corners.insert(0b00100011, 1);
    inp.nbrs_to_corners.insert(0b00100100, 3 * 2);
    inp.nbrs_to_corners.insert(0b00100101, 3 * 2);
    inp.nbrs_to_corners.insert(0b00100110, 1);
    inp.nbrs_to_corners.insert(0b00100111, 2);
    inp.nbrs_to_corners.insert(0b00101000, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00101001, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00101010, 2);
    inp.nbrs_to_corners.insert(0b00101011, 3);
    inp.nbrs_to_corners.insert(0b00101100, 3 + 2);
    inp.nbrs_to_corners.insert(0b00101101, 3 + 2);
    inp.nbrs_to_corners.insert(0b00101110, 1);
    inp.nbrs_to_corners.insert(0b00101111, 2);
    inp.nbrs_to_corners.insert(0b00110000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00110001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00110010, 1);
    inp.nbrs_to_corners.insert(0b00110011, 2);
    inp.nbrs_to_corners.insert(0b00110100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00110101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b00110110, 2);
    inp.nbrs_to_corners.insert(0b00110111, 3);
    inp.nbrs_to_corners.insert(0b00111000, 3);
    inp.nbrs_to_corners.insert(0b00111001, 3);
    inp.nbrs_to_corners.insert(0b00111010, 1);
    inp.nbrs_to_corners.insert(0b00111011, 2);
    inp.nbrs_to_corners.insert(0b00111100, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00111101, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b00111110, 0);
    inp.nbrs_to_corners.insert(0b00111111, 1);
    inp.nbrs_to_corners.insert(0b01000000, 3 * 4);
    inp.nbrs_to_corners.insert(0b01000001, 3 * 4);
    inp.nbrs_to_corners.insert(0b01000010, 3 * 2);
    inp.nbrs_to_corners.insert(0b01000011, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01000100, 3 * 4);
    inp.nbrs_to_corners.insert(0b01000101, 3 * 4);
    inp.nbrs_to_corners.insert(0b01000110, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01000111, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01001000, 3 * 2);
    inp.nbrs_to_corners.insert(0b01001001, 3 * 2);
    inp.nbrs_to_corners.insert(0b01001010, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b01001011, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01001100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01001101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01001110, 3);
    inp.nbrs_to_corners.insert(0b01001111, 3 + 1);
    inp.nbrs_to_corners.insert(0b01010000, 3 * 4);
    inp.nbrs_to_corners.insert(0b01010001, 3 * 4);
    inp.nbrs_to_corners.insert(0b01010010, 3 * 2);
    inp.nbrs_to_corners.insert(0b01010011, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01010100, 3 * 4);
    inp.nbrs_to_corners.insert(0b01010101, 3 * 4);
    inp.nbrs_to_corners.insert(0b01010110, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01010111, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01011000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01011001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01011010, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01011011, 3 * 1 + 3);
    inp.nbrs_to_corners.insert(0b01011100, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01011101, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01011110, 3 + 1);
    inp.nbrs_to_corners.insert(0b01011111, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01100000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01100001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01100010, 1);
    inp.nbrs_to_corners.insert(0b01100011, 2);
    inp.nbrs_to_corners.insert(0b01100100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01100101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b01100110, 2);
    inp.nbrs_to_corners.insert(0b01100111, 3);
    inp.nbrs_to_corners.insert(0b01101000, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01101001, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01101010, 3);
    inp.nbrs_to_corners.insert(0b01101011, 1 * 4);
    inp.nbrs_to_corners.insert(0b01101100, 3 + 3);
    inp.nbrs_to_corners.insert(0b01101101, 3 + 3);
    inp.nbrs_to_corners.insert(0b01101110, 1 * 2);
    inp.nbrs_to_corners.insert(0b01101111, 1 * 3);
    inp.nbrs_to_corners.insert(0b01110000, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01110001, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01110010, 2);
    inp.nbrs_to_corners.insert(0b01110011, 1 * 3);
    inp.nbrs_to_corners.insert(0b01110100, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01110101, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b01110110, 1 * 3);
    inp.nbrs_to_corners.insert(0b01110111, 1 * 4);
    inp.nbrs_to_corners.insert(0b01111000, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b01111001, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b01111010, 1 * 2);
    inp.nbrs_to_corners.insert(0b01111011, 1 * 3);
    inp.nbrs_to_corners.insert(0b01111100, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01111101, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b01111110, 1);
    inp.nbrs_to_corners.insert(0b01111111, 2);
    inp.nbrs_to_corners.insert(0b10000000, 3 * 2);
    inp.nbrs_to_corners.insert(0b10000001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b10000010, 3 + 1);
    inp.nbrs_to_corners.insert(0b10000011, 3);
    inp.nbrs_to_corners.insert(0b10000100, 3 * 2);
    inp.nbrs_to_corners.insert(0b10000101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b10000110, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b10000111, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b10001000, 0);
    inp.nbrs_to_corners.insert(0b10001001, 1);
    inp.nbrs_to_corners.insert(0b10001010, 2);
    inp.nbrs_to_corners.insert(0b10001011, 1);
    inp.nbrs_to_corners.insert(0b10001100, 1);
    inp.nbrs_to_corners.insert(0b10001101, 2);
    inp.nbrs_to_corners.insert(0b10001110, 1);
    inp.nbrs_to_corners.insert(0b10001111, 0);
    inp.nbrs_to_corners.insert(0b10010000, 3 * 2);
    inp.nbrs_to_corners.insert(0b10010001, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b10010010, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b10010011, 3 * 1);
    inp.nbrs_to_corners.insert(0b10010100, 3 * 2);
    inp.nbrs_to_corners.insert(0b10010101, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b10010110, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b10010111, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b10011000, 1);
    inp.nbrs_to_corners.insert(0b10011001, 2);
    inp.nbrs_to_corners.insert(0b10011010, 3);
    inp.nbrs_to_corners.insert(0b10011011, 2);
    inp.nbrs_to_corners.insert(0b10011100, 2);
    inp.nbrs_to_corners.insert(0b10011101, 3);
    inp.nbrs_to_corners.insert(0b10011110, 2);
    inp.nbrs_to_corners.insert(0b10011111, 1);
    inp.nbrs_to_corners.insert(0b10100000, 3 + 1);
    inp.nbrs_to_corners.insert(0b10100001, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b10100010, 2);
    inp.nbrs_to_corners.insert(0b10100011, 1);
    inp.nbrs_to_corners.insert(0b10100100, 3 + 1);
    inp.nbrs_to_corners.insert(0b10100101, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b10100110, 1 * 3);
    inp.nbrs_to_corners.insert(0b10100111, 2);
    inp.nbrs_to_corners.insert(0b10101000, 2);
    inp.nbrs_to_corners.insert(0b10101001, 3);
    inp.nbrs_to_corners.insert(0b10101010, 4);
    inp.nbrs_to_corners.insert(0b10101011, 3);
    inp.nbrs_to_corners.insert(0b10101100, 3);
    inp.nbrs_to_corners.insert(0b10101101, 4);
    inp.nbrs_to_corners.insert(0b10101110, 3);
    inp.nbrs_to_corners.insert(0b10101111, 2);
    inp.nbrs_to_corners.insert(0b10110000, 3 + 2);
    inp.nbrs_to_corners.insert(0b10110001, 3 + 3);
    inp.nbrs_to_corners.insert(0b10110010, 3);
    inp.nbrs_to_corners.insert(0b10110011, 2);
    inp.nbrs_to_corners.insert(0b10110100, 3 + 2);
    inp.nbrs_to_corners.insert(0b10110101, 3 + 3);
    inp.nbrs_to_corners.insert(0b10110110, 4);
    inp.nbrs_to_corners.insert(0b10110111, 3);
    inp.nbrs_to_corners.insert(0b10111000, 1);
    inp.nbrs_to_corners.insert(0b10111001, 2);
    inp.nbrs_to_corners.insert(0b10111010, 3);
    inp.nbrs_to_corners.insert(0b10111011, 2);
    inp.nbrs_to_corners.insert(0b10111100, 2);
    inp.nbrs_to_corners.insert(0b10111101, 3);
    inp.nbrs_to_corners.insert(0b10111110, 2);
    inp.nbrs_to_corners.insert(0b10111111, 1);
    inp.nbrs_to_corners.insert(0b11000000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b11000001, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b11000010, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b11000011, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b11000100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b11000101, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b11000110, 3 * 1 + 3);
    inp.nbrs_to_corners.insert(0b11000111, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b11001000, 1);
    inp.nbrs_to_corners.insert(0b11001001, 2);
    inp.nbrs_to_corners.insert(0b11001010, 3);
    inp.nbrs_to_corners.insert(0b11001011, 2);
    inp.nbrs_to_corners.insert(0b11001100, 2);
    inp.nbrs_to_corners.insert(0b11001101, 3);
    inp.nbrs_to_corners.insert(0b11001110, 2);
    inp.nbrs_to_corners.insert(0b11001111, 1);
    inp.nbrs_to_corners.insert(0b11010000, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b11010001, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b11010010, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b11010011, 3 * 1 + 1);
    inp.nbrs_to_corners.insert(0b11010100, 3 * 2 + 1);
    inp.nbrs_to_corners.insert(0b11010101, 3 * 2 + 2);
    inp.nbrs_to_corners.insert(0b11010110, 3 * 1 + 3);
    inp.nbrs_to_corners.insert(0b11010111, 3 * 1 + 2);
    inp.nbrs_to_corners.insert(0b11011000, 2);
    inp.nbrs_to_corners.insert(0b11011001, 3);
    inp.nbrs_to_corners.insert(0b11011010, 4);
    inp.nbrs_to_corners.insert(0b11011011, 3);
    inp.nbrs_to_corners.insert(0b11011100, 3);
    inp.nbrs_to_corners.insert(0b11011101, 4);
    inp.nbrs_to_corners.insert(0b11011110, 3);
    inp.nbrs_to_corners.insert(0b11011111, 2);
    inp.nbrs_to_corners.insert(0b11100000, 3);
    inp.nbrs_to_corners.insert(0b11100001, 3 + 1);
    inp.nbrs_to_corners.insert(0b11100010, 1);
    inp.nbrs_to_corners.insert(0b11100011, 0);
    inp.nbrs_to_corners.insert(0b11100100, 3);
    inp.nbrs_to_corners.insert(0b11100101, 3 + 1);
    inp.nbrs_to_corners.insert(0b11100110, 2);
    inp.nbrs_to_corners.insert(0b11100111, 1);
    inp.nbrs_to_corners.insert(0b11101000, 1);
    inp.nbrs_to_corners.insert(0b11101001, 2);
    inp.nbrs_to_corners.insert(0b11101010, 3);
    inp.nbrs_to_corners.insert(0b11101011, 2);
    inp.nbrs_to_corners.insert(0b11101100, 2);
    inp.nbrs_to_corners.insert(0b11101101, 3);
    inp.nbrs_to_corners.insert(0b11101110, 2);
    inp.nbrs_to_corners.insert(0b11101111, 1);
    inp.nbrs_to_corners.insert(0b11110000, 3 + 1);
    inp.nbrs_to_corners.insert(0b11110001, 3 + 2);
    inp.nbrs_to_corners.insert(0b11110010, 2);
    inp.nbrs_to_corners.insert(0b11110011, 1);
    inp.nbrs_to_corners.insert(0b11110100, 3 + 1);
    inp.nbrs_to_corners.insert(0b11110101, 3 + 2);
    inp.nbrs_to_corners.insert(0b11110110, 3);
    inp.nbrs_to_corners.insert(0b11110111, 2);
    inp.nbrs_to_corners.insert(0b11111000, 0);
    inp.nbrs_to_corners.insert(0b11111001, 1);
    inp.nbrs_to_corners.insert(0b11111010, 2);
    inp.nbrs_to_corners.insert(0b11111011, 1);
    inp.nbrs_to_corners.insert(0b11111100, 1);
    inp.nbrs_to_corners.insert(0b11111101, 2);
    inp.nbrs_to_corners.insert(0b11111110, 1);
    inp.nbrs_to_corners.insert(0b11111111, 0);

    return inp;
}

impl Inp {
    fn get_region(&mut self, p: Pos) -> Region {
        let mut region = Region {
            letter: self.lines[p.y as usize].chars().nth(p.x as usize).unwrap(),
            p: HashMap::new(),
        };
        region.p.insert(p, true);
        self.visited[p.y as usize][p.x as usize] = true;
        let mut edge: Vec<Pos> = Vec::new();
        edge.push(p);
        while edge.len() > 0 {
            let mut newedge = Vec::new();
            for p in edge {
                for n in p.nbrs() {
                    if n.x >= 0 && n.x < self.lines[0].len() as i32 {
                        if n.y >= 0 && n.y < self.lines.len() as i32 {
                            if self.lines[n.y as usize].chars().nth(n.x as usize).unwrap()
                                == region.letter
                            {
                                if !self.visited[n.y as usize][n.x as usize] {
                                    newedge.push(n);
                                    self.visited[n.y as usize][n.x as usize] = true;
                                }
                            }
                        }
                    }
                }
            }
            for n in newedge.iter() {
                region.p.insert(*n, true);
            }
            edge = newedge;
        }
        return region;
    }

    fn get_regions(&mut self) -> Vec<Region> {
        let mut regions = Vec::new();
        for y in 0..self.lines.len() {
            for x in 0..self.lines[0].len() {
                if !self.visited[y][x] {
                    regions.push(self.get_region(Pos {
                        x: x as i32,
                        y: y as i32,
                    }));
                }
            }
        }
        self.visited = vec![vec![false; self.lines[0].len()]; self.lines.len()];
        return regions;
    }

    fn metrics(&mut self) -> i32 {
        let mut res = 0;
        for r in self.get_regions() {
            res += r.cost();
        }
        return res;
    }

    fn getcorners3(&self, r: Region, trace: bool) -> i32 {
        let mut corners3: i32 = 0;
        for p in r.p.iter() {
            let nbrsmap = &r.nbrsmap(p.0);
            corners3 += *self.nbrs_to_corners.get(&nbrsmap).unwrap_or(&0) as i32;
            if trace {
                println!(
                    "p: {:?} map:{:08b} corners: {}",
                    p,
                    nbrsmap,
                    *self.nbrs_to_corners.get(&nbrsmap).unwrap_or(&0)
                )
            }
        }
        return corners3;
    }
    fn metrics2(&mut self) -> i32 {
        let mut res = 0;
        for r in self.get_regions() {
            let mut corners3: i32 = 0;
            for p in r.p.iter() {
                let nbrsmap = &r.nbrsmap(p.0);
                corners3 += *self.nbrs_to_corners.get(&nbrsmap).unwrap_or(&0) as i32;
            }
            if corners3 % 3 != 0 {
                println!("oops {:?}", r);
                println!("{}", r.string());
                let mut corners3: i32 = 0;
                for p in r.p.iter() {
                    let nbrsmap = &r.nbrsmap(p.0);
                    corners3 += *self.nbrs_to_corners.get(&nbrsmap).unwrap_or(&0) as i32;
                    println!(
                        "p: {:?} map:{:08b} corners: {}",
                        p,
                        nbrsmap,
                        *self.nbrs_to_corners.get(&nbrsmap).unwrap_or(&0)
                    )
                }
                panic!("AAAA")
            }
            res += r.area() * corners3 / 3;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut inp = read_file("test.txt");
        let mut rng = thread_rng();
        for i in 1..10000 {
            let mut r: Region = Region {
                p: HashMap::new(),
                letter: 'A',
            };
            for j in 1..10 {
                r.p.insert(
                    Pos {
                        x: rng.gen_range(0..4),
                        y: rng.gen_range(0..4),
                    },
                    true,
                );
            }
            let corners3 = inp.getcorners3(r.clone(), false);
            if corners3 % 3 != 0 {
                println!("oops \n{}", r.string());
                let _ = inp.getcorners3(r.clone(), true);

                panic!("!!!")
            }
        }
    }
    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 1930);
        assert_eq!(read_file("test.txt").metrics2(), 1206);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
