// use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    lines: Vec<String>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp { lines: vec![] };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().for_each(|line| {
        res.lines.push(line.to_string());
    });
    return res;
}

const dirs: [&[i32; 2]; 8] = [
    &[1, 0],
    &[1, 1],
    &[0, 1],
    &[-1, 1],
    &[-1, 0],
    &[-1, -1],
    &[0, -1],
    &[1, -1],
];

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        println!("{} {}", u, i);
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

impl Inp {
    fn cango(&self, d: [i32; 2], r: usize, c: usize) -> bool {
        if d[0] > 0 && c > self.lines[r].len() - 4 {
            return false;
        }
        if d[0] < 0 && c < 3 {
            return false;
        }
        if d[1] > 0 && r > self.lines.len() - 4 {
            return false;
        }
        if d[1] < 0 && r < 3 {
            return false;
        }
        return true;
    }
    fn isXMAS(&self, d: [i32; 2], r: usize, c: usize) -> bool {
        if self.lines[add(r, d[1])].as_bytes()[add(c, d[0])] != "M".as_bytes()[0] {
            return false;
        }
        if self.lines[add(r, d[1] * 2)].as_bytes()[add(c, d[0] * 2)] != "A".as_bytes()[0] {
            return false;
        }
        if self.lines[add(r, d[1] * 3)].as_bytes()[add(c, d[0] * 3)] != "S".as_bytes()[0] {
            return false;
        }
        return true;
    }
    // M S
    //  A
    // M S
    fn isXMAS2(&self, r: usize, c: usize) -> bool {
        let nbrs = [
            self.lines[r - 1].as_bytes()[c - 1],
            self.lines[r - 1].as_bytes()[c + 1],
            self.lines[r + 1].as_bytes()[c + 1],
            self.lines[r + 1].as_bytes()[c - 1],
        ];
        if nbrs == ['M' as u8, 'S' as u8, 'S' as u8, 'M' as u8] {
            return true;
        }
        if nbrs == ['M' as u8, 'M' as u8, 'S' as u8, 'S' as u8] {
            return true;
        }
        if nbrs == ['S' as u8, 'S' as u8, 'M' as u8, 'M' as u8] {
            return true;
        }
        if nbrs == ['S' as u8, 'M' as u8, 'M' as u8, 'S' as u8] {
            return true;
        }
        return false;
    }
    fn metrics(&self) -> i32 {
        let mut res = 0;
        for (r, line) in self.lines.iter().enumerate() {
            for (c, v) in line.chars().enumerate() {
                if v == 'X' {
                    for dir in dirs {
                        if self.cango(*dir, r, c) {
                            if self.isXMAS(*dir, r, c) {
                                res += 1;
                            }
                        }
                    }
                }
            }
        }
        return res;
    }
    fn metrics2(&self) -> i32 {
        let mut res = 0;
        for (r, line) in self.lines.iter().enumerate() {
            for (c, v) in line.chars().enumerate() {
                if v == 'A'
                    && r > 0
                    && c > 0
                    && r < self.lines.len() - 1
                    && c < self.lines[r].len() - 1
                {
                    if self.isXMAS2(r, c) {
                        res += 1;
                    }
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
        assert_eq!(read_file("test.txt").metrics(), 18);
        assert_eq!(read_file("test.txt").metrics2(), 9);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
