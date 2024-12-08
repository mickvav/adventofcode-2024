// use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Inp {
    line: String,
}

fn read_file(filename: &str) -> Inp {
    return Inp {
        line: fs::read_to_string(filename).expect("Something went wrong reading the file"),
    };
}

impl Inp {
    fn metrics(&self) -> i32 {
        let re =
            Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

        let mut res = 0;
        for (_, [ m1, m2]) in re.captures_iter(&self.line).map(|c| c.extract()) {
            res += m1.parse::<i32>().unwrap() * m2.parse::<i32>().unwrap();
        }
        return res;
    }
    fn metrics2(&self) -> i32 {
        let re =
            Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))|(do\(()()\)|don't\(()()\))").unwrap();

        let mut res = 0;
        let mut enabled = true;
        for (_, [v, m1, m2]) in re.captures_iter(&self.line).map(|c| c.extract()) {
            if v == "don't()" {
                enabled = false;
                continue;
            }
            if v == "do()" {
                enabled = true;
                continue;
            }
            if !enabled {
                continue;
            }
            res += m1.parse::<i32>().unwrap() * m2.parse::<i32>().unwrap();
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 161);
        assert_eq!(read_file("test2.txt").metrics2(), 48);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
