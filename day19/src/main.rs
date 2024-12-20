use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Inp {
    towels: Vec<String>,
    lines: Vec<String>,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines = buf.lines();
    let mut res = Inp {
        towels: vec![],
        lines: vec![],
    };
    res.towels = lines.next().unwrap().split(", ").map(|s| s.to_string()).collect();

    for line in lines {
        res.lines.push(line.to_string());
    }
    return res;
}

impl Inp {
    fn metrics(&self) -> i32 {
        let mut restring = String::new();
        restring.push_str("^(");
        restring.push_str(&self.towels.join("|"));
        restring.push_str(")+$");
        let re = Regex::new(&restring).unwrap();
        let mut res = 0;
        for l in self.lines.iter() {
            if re.is_match(&l) {
                res += 1;
            }
        }
       return res;
    }
    fn metrics2(&self) -> i64 {
        let mut res = 0;
        let mut hash: HashMap<String, i64> = HashMap::new();
        for l in self.lines.iter() {
            println!("{}", l);
            res += self.countarrangements(&l, &mut hash);
        }
       return res;
    }
    fn countarrangements(&self, s: &str, hash: &mut HashMap<String, i64>) -> i64 {
        if hash.contains_key(s) {
            return hash[s];
        }
   //     println!(">{}", s);
        let mut res=0;
        for t in self.towels.iter() {
            if s == t {
                res += 1;
            } else {
                if s.len() < t.len() {
                    continue;
                }
                if &s[0..t.len()] == t.as_str() {
                    res += self.countarrangements(&s[t.len()..], hash);
                }
            }
        }
        hash.insert(s.to_string(), res);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 6);
        assert_eq!(read_file("test.txt").metrics2(), 16);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
