use std::collections::HashMap;
use std::fs;
use std::fmt;


#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    r: usize,
    c: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({:#?})", self)
    }
}
#[derive(Debug)]
struct Inp {
    rows: usize,
    cols: usize,
    pos: HashMap<u8, Vec<Pos>>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp {
        rows: 0,
        cols: 0,
        pos: HashMap::new(),
    };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents.lines().for_each(|line| {
        res.cols = line.len();
        res.rows += 1;
        line.chars().enumerate().for_each(|(i, c)| {
            if c.is_alphabetic() || c.is_digit(10) {
                res.pos.entry(c as u8).or_insert(vec![]).push(Pos {
                    r: res.rows - 1,
                    c: i,
                });
            }
        });
    });
    return res;
}

fn boxedshift(x1: usize, x2: usize, max: usize) -> (bool, usize) {
    let ix1 = x1 as i32;
    let ix2 = x2 as i32;
    let ix3 = ix1 + (ix2 - ix1) * 2;
    if ix3 < 0 {
        return (false, 0);
    }
    if ix3 >= max as i32 {
        return (false, 0);
    }
    return (true, ix3 as usize);
}
impl Inp {
    fn metrics(&self) -> usize {
        let mut loc: HashMap<Pos, u8> = HashMap::new();
        for (u, v) in self.pos.iter() {
            println!("> {} {}", u, v.len());
            for p1 in v {
                for p2 in v {
                    if p1 != p2 {
                        let (b1, r1) = boxedshift(p1.r, p2.r, self.rows);
                        let (b2, c1) = boxedshift(p1.c, p2.c, self.cols);
                        if b1 && b2 {
                            loc.entry(Pos { r: r1, c: c1 }).or_insert(*u);
                            println!("{} {}", r1, c1)
                        }
                    }
                }
            }
        }
        return loc.len();
    }
    fn metrics2(&self) -> usize {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 14);
        assert_eq!(read_file("test.txt").metrics2(), 0);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
