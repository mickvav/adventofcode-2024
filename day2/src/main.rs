// use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    card: Vec<Vec<i32>>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp { card: vec![] };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().for_each(|line| {
        let parts = line.split(" ");
        let mut row = vec![];
        parts.for_each(|part| {
            row.push(part.parse::<i32>().unwrap());
        });
        res.card.push(row);
    });
    return res;
}

fn is_safe(row: &Vec<i32>) -> bool {
    let mut dir = 0;
    let mut prev = 0;
    let mut nonsafe = false;
    row.iter().for_each(|v| {
        if prev == 0 {
            prev = *v;
        } else {
            if dir == 0 {
                if *v > prev {
                    dir = 1;
                } else {
                    if *v < prev {
                        dir = -1;
                    } else {
                        nonsafe = true;
                        return;
                    };
                };
            } else {
                if dir == 1 {
                    if *v < prev {
                        nonsafe = true;
                        return;
                    }
                } else {
                    if *v >= prev {
                        nonsafe = true;
                        return;
                    }
                }
            }
            if (*v - prev).abs() < 1 {
                nonsafe = true;
                return;
            }
            if (*v - prev).abs() > 3 {
                nonsafe = true;
                return;
            };
            prev = *v;
        };
    });
    return !nonsafe;
}
impl Inp {
    fn metrics(&self) -> i32 {
        let mut res = 0;
        self.card.iter().for_each(|row| {
            if is_safe(row) {
                res += 1;
                println!("{:?}", row);
            }
        });
        return res;
    }
    fn metrics2(&self) -> i32 {
        let mut res = 0;
        self.card.iter().for_each(|row| {
            for i in 0..row.len() {
                let mut r1 = row.clone();
                r1.remove(i);
                if is_safe(&r1) {
                    res += 1;
                    println!("{:?}", r1);
                    return;
                }
            }
        });
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inp = read_file("test.txt");
        let result = inp.metrics();
        assert_eq!(result, 2);
        assert_eq!(inp.metrics2(), 4);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
