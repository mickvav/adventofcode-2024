use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    seeds: Vec<i64>,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = buf.lines();
    let mut res = Inp {
        seeds: Vec::new(),
    };

    for line in lines {
        res.seeds.push(line.parse::<i64>().unwrap());
    }
    println!("{:?}", res);
    return res;
}

fn mix(what: i64, secret: i64) -> i64 {
    return what ^ secret;
}

fn prune(what: i64) -> i64 {
    return what % 16777216;
}

fn next(secret: i64) -> i64 {
    let s = secret << 6; // *64
    let s1 = prune(mix(s,secret));
    let s2 = prune(mix(s1 >> 5, s1)); // /32
    let s3 = prune(mix(s2 << 11, s2)); // *2048
    return s3
}

fn next2k(secret: i64) -> i64 {
    let mut res = secret;
    for _ in 0..2000 {
        res = next(res);
    }
    return res;
}
impl Inp {
   fn metrics(&self) -> i64 {
        let mut res = 0;
        for m in self.seeds.iter() {
            res += next2k(*m);
        }
        return res;
    }
    fn metrics2(&self) -> i64 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 37327623);
        assert_eq!(read_file("test.txt").metrics2(), 0);
    }

    #[test]
    fn simple() {
        assert_eq!(
            next(123), 15887950
        );
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
