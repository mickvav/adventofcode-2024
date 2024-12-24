use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Inp {
    levelmap: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
    distance: Vec<Vec<i32>>,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines = buf.lines();
    let mut res = Inp {
        levelmap: Vec::new(),
        start: Pos{row: 0, col: 0},
        end: Pos{row: 0, col: 0},
        distance: Vec::new(),
    };

    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
            if c == 'S' {
                res.start = Pos{row:res.levelmap.len(), col:row.len() - 1};
            }
            if c == 'E' {
                res.end = Pos{row:res.levelmap.len(), col: row.len() - 1};
            }
        }
        let mut dr = Vec::new();
        for _ in 0..row.len() {
            dr.push(-1);
        }
        res.levelmap.push(row);
        res.distance.push(dr);
    }
    res.walk();
   return res;
}  


impl Inp {
    fn walk(&mut self) {
        self.distance[self.start.row][self.start.col] = 0;
        let mut row = self.start.row;
        let mut col = self.start.col;
        let mut curdist = 0;
        self.distance[row][col] = 0;
        while (row != self.end.row) || (col != self.end.col) {
            curdist += 1;
            let n = self.nbrs(row, col);
            for p in n.iter() {
                if self.levelmap[p.row][p.col] == '#' {
                    continue;
                }
                if self.distance[p.row][p.col] == -1 {
                    self.distance[p.row][p.col] = curdist;
                    row = p.row;
                    col = p.col;
                    break;
                }
            }   
        }
        // print distances
        for row in 0..self.levelmap.len() {
            for col in 0..self.levelmap[0].len() {
                print!("{:02} ", self.distance[row][col]);
            }
            println!("");
        }

    }

    fn nbrs(&self, row: usize, col: usize) -> Vec<Pos> {
        let mut res = Vec::new();
        if row > 0 {
            res.push(Pos{row: row - 1, col: col});
        }
        if row < self.levelmap.len() - 1 {
            res.push(Pos{row: row + 1, col: col});
        }
        if col > 0 {
            res.push(Pos{row: row, col: col - 1});
        }
        if col < self.levelmap[0].len() - 1 {
            res.push(Pos{row: row, col: col + 1});
        }
        return res;
    }
    fn nbrs2(&self, row: usize, col: usize) -> Vec<Pos> {
        let mut res = Vec::new();
        if row > 1 {
            res.push(Pos{row: row - 2, col: col});
        }
        if row < self.levelmap.len() - 2 {
            res.push(Pos{row: row + 2, col: col});
        }
        if col > 1 {
            res.push(Pos{row: row, col: col - 2});
        }
        if col < self.levelmap[0].len() - 2 {
            res.push(Pos{row: row, col: col + 2});
        }
        return res;
    }

    fn nbrs20(&self, row: usize, col: usize) -> Vec<Pos> {
        let mut res = Vec::new();
        for r in (row as i32) - 20..(row as i32) + 21 {
            for c in (col as i32) - 20..(col as i32) + 21 {
                if r < 0 || r >= self.levelmap.len() as i32 || c < 0 || c >= self.levelmap[0].len() as i32 {
                    continue;
                }
                if (r - row as i32).abs() + (c - col as i32).abs() > 20 {
                    continue;
                }
              /*   if (r - row as i32).abs() + (c - col as i32).abs() <= 1 {
                    continue;
                }
                if self.levelmap[r as usize][c as usize] == '#' {
                    continue;
                }*/
                res.push(Pos{row: r as usize, col: c as usize})
            }
        }
       /* println!("row: {}, col: {}", row, col);
        println!("res:");
        println!("{:?}", res); */
        return res;
    }


    fn metrics(&self, threshold : i32) -> i32 {
        let mut  res = 0;
        for row in 0..self.levelmap.len() {
            for col in 0..self.levelmap[0].len() {
                if self.levelmap[row][col] == '#' {
                    continue;
                }
                for p in self.nbrs2(row, col) {
                    if self.levelmap[p.row][p.col] == '#' {
                        continue;
                    }
                    if self.distance[p.row][p.col] == -1 {
                        continue;
                    }
                    if self.distance[p.row][p.col] > self.distance[row][col] + threshold {
                        res +=1;
                    }
                }
            }
        }
        return res;
    }
    fn metrics2(&self, threshold : i32) -> i32 {
        let mut res = 0;
         for row in 0..self.levelmap.len() {
            for col in 0..self.levelmap[0].len() {
                if self.levelmap[row][col] == '#' {
                    continue;
                }
                for p in self.nbrs20(row, col) {
                    if self.levelmap[p.row][p.col] == '#' {
                        continue;
                    }
                    if self.distance[p.row][p.col] == -1 {
                        continue;
                    }
                    let cheatcost = (p.row as i32 - row as i32).abs() + (p.col as i32 - col as i32).abs();

                    if self.distance[p.row][p.col] >= self.distance[row][col] + cheatcost + threshold {
                        res +=1;
                      //  println!("cheat: {} {} {} {}", row, col, p.row, p.col)
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
        assert_eq!(read_file("test.txt").metrics(10), 10);
        assert_eq!(read_file("test.txt").metrics2(72), 29);
        assert_eq!(read_file("test.txt").metrics2(70), 29 + 12);
        assert_eq!(read_file("test.txt").metrics2(68), 29 + 12 + 14);
        assert_eq!(read_file("test.txt").metrics2(66), 29 + 12 + 14 + 12);
        assert_eq!(read_file("test.txt").metrics2(64), 29 + 12 + 14 + 12 + 19);
        assert_eq!(read_file("test.txt").metrics2(62), 29 + 12 + 14 + 12 + 19 + 20);
        assert_eq!(read_file("test.txt").metrics2(60), 29 + 12 + 14 + 12 + 19 + 20 + 23);
        assert_eq!(read_file("test.txt").metrics2(58), 29 + 12 + 14 + 12 + 19 + 20 + 23 + 25);
        assert_eq!(read_file("test.txt").metrics2(56), 29 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39);
        assert_eq!(read_file("test.txt").metrics2(54), 29 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29);
        assert_eq!(read_file("test.txt").metrics2(52), 29 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29 + 31);
        assert_eq!(read_file("test.txt").metrics2(50), 29 + 12 + 14 + 12 + 19 + 20 + 23 + 25 + 39 + 29 + 31 + 32);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics(100));
    println!("Part2 {:?}", v.metrics2(100));
    println!("Part2 {:?}", v.metrics2(100));
}
