// use std::collections::HashMap;
// use std::fmt;
use std::fs;

#[derive(Debug)]
struct Inp {
    blocks: Vec<i32>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp { blocks: vec![] };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut isblock = true;
    let mut blockn = 0;
    contents.chars().for_each(|c| {
        if c.is_digit(10) {
            let v = c.to_digit(10).unwrap() as i32;
            println!("{} {} {}", v, isblock, blockn);
            if isblock {
                for _i in 0..v {
                    res.blocks.push(blockn);
                }
                blockn += 1;
                isblock = false;
            } else {
                for _i in 0..v {
                    res.blocks.push(-1);
                }
                isblock = true;
            }
        }
    });
    return res;
}
fn firstempty(b: &Vec<i32>) -> usize {
    for i in 0..b.len() {
        if b[i] == -1 {
            return i;
        }
    }
    return b.len();
}
fn firstemptyfrom(b: &Vec<i32>, i: usize) -> usize {
    for j in i..b.len() {
        if b[j] == -1 {
            return j;
        }
    }
    return b.len();
}

fn lastnonempty(b: &Vec<i32>) -> usize {
    for i in (0..b.len()).rev() {
        if b[i] != -1 {
            return i;
        }
    }
    return 0;
}

fn lastnonemptyfrom(b: &Vec<i32>, i: usize) -> usize {
    for j in (0..i).rev() {
        if b[j] != -1 {
            return j;
        }
    }
    return 0;
}

fn checksum(b: &Vec<i32>) -> i64 {
    let mut res = 0;
    for i in 0..b.len() {
        if b[i] != -1 {
            res += (b[i] as i64) * (i as i64);
        }
    }
    return res;
}
impl Inp {
    fn defrag(&self) -> Vec<i32> {
        let mut b = self.blocks.clone();
        let mut i = firstempty(&b);
        let mut j = lastnonempty(&b);
        println!(" > {:?}", b);
        println!(" i {:?}", i);
        println!(" j {:?}", j);
        while i < j {
            let bl = b[j];
            b[j] = b[i];
            b[i] = bl;
            i = firstemptyfrom(&b, i);
            j = lastnonemptyfrom(&b, j);
            //   println!("{:?}", b);
        }
        return b;
    }
    fn blockstart(&self, b: &Vec<i32>, i: usize) -> usize {
        // find first index of block, last index of which is i
        for j in (0..i).rev() {
            if b[j] != b[i] {
                return j + 1;
            }
        }
        return 0;
    }
    fn findemptyblock(&self, blocks: &Vec<i32>, i: usize, len: usize) -> usize {
        let mut start = i;
        'outer: while start < blocks.len() {
            for j in start..start + len {
                if j >= blocks.len() {
                    break 'outer;
                }
                if blocks[j] != -1 {
                    start = firstemptyfrom(blocks, j);
                    continue 'outer;
                }
            }
            return start;
        }
        return self.blocks.len();
    }
    fn printblocks(&self, b: &Vec<i32>) {
        for c in b.iter() {
            if *c == -1 {
                print!(".");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    fn defrag2(&self) -> Vec<i32> {
        let mut b = self.blocks.clone();
        let mut i = firstempty(&b);
        let mut j = lastnonempty(&b);
        self.printblocks(&b);
        while i < j {
            let j1 = self.blockstart(&b, j);
            let len = j - j1 + 1;
            let i1 = self.findemptyblock(&b, i, len);
            if i1 < j1 {
                for p in i1..i1 + len {
                    b[p] = b[j];
                    b[j1 + p - i1] = -1;
                }
            }
            j = lastnonemptyfrom(&b, j1);
            i = firstemptyfrom(&b, i);
        }
        return b;
    }
    fn metrics(&self) -> i64 {
        let d = self.defrag();
        return checksum(&d);
    }
    fn metrics2(&self) -> i64 {
        let d = self.defrag2();
        return checksum(&d);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 1928);
        assert_eq!(read_file("test.txt").metrics2(), 2858);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
