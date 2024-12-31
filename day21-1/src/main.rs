use memoize::memoize;
use std::fs;

#[derive(Debug)]
struct Inp {
    lines: Vec<String>,
}

fn read_file(filename: &str) -> Inp {
    let mut inp = Inp { lines: vec![] };

    let line = fs::read_to_string(filename).expect("Something went wrong reading the file");
    inp.lines = line.lines().map(|s| s.to_string()).collect();
    return inp;
}

fn testseq(s: String, d: i32, trace: bool) -> i64 {
    let mut res = 0;
    let mut tracedchars = Vec::new();
    let mut p = 'A';
    for i in 0..s.len() {
        let (b, mut t) = buttons(s.chars().nth(i).unwrap(), p, d, trace);
        tracedchars.append(&mut t);
        res += b;
        p = s.chars().nth(i).unwrap();
    }
    if trace {
        println!("tracelen: {}", tracedchars.len());
        println!(
            "trace: {}",
            tracedchars
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
                .join("")
                .to_string()
        );
    }
    println!("{}", res);
    return res;
}
//    ^ A
//  < v >
//

fn pickpath(options: Vec<Vec<char>>, d: i32, trace: bool) -> (i64, Vec<char>) {
    let mut chars: Vec<char> = Vec::new();
    let mut minlength = usize::MAX;

    let mut res = 0;
    for o in options {
        let mut tmp: i64 = 0;
        let mut p = 'A';
        let mut tmpchars = Vec::new();
        for c in o {
            let (t, mut b) = buttons(c, p, d, trace);
            tmp += t;
            if trace {
                tmpchars.append(&mut b);
            }
            p = c;
        }
        if (tmp as usize) < minlength {
            minlength = tmp as usize;
            res = tmp;
            if trace {
                chars = tmpchars;
            }
        }
    }
    return (res, chars);
}

#[memoize]
fn paths(t: (char, char)) -> Vec<Vec<char>> {
    match t {
        ('0', 'A') => {
            return vec![vec!['>', 'A']];
        }
        ('A', '0') => {
            return vec![vec!['<', 'A']];
        }
        ('0', '1') => {
            return vec![vec!['^', '<', 'A']];
        }
        ('1', '0') => {
            return vec![vec!['>', 'v', 'A']];
        }
        ('0', '2') => {
            return vec![vec!['^', 'A']];
        }
        ('2', '0') => {
            return vec![vec!['v', 'A']];
        }
        ('0', '3') => {
            return vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']];
        }
        ('3', '0') => {
            return vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']];
        }
        ('0', '4') => {
            return vec![vec!['^', '<', '^', 'A'], vec!['^', '^', '<', 'A']];
        }
        ('4', '0') => {
            return vec![vec!['v', '>', 'v', 'A'], vec!['>', 'v', 'v', 'A']];
        }
        ('0', '5') => {
            return vec![vec!['^', '^', 'A']];
        }
        ('5', '0') => {
            return vec![vec!['v', 'v', 'A']];
        }
        ('0', '6') => {
            return vec![
                vec!['^', '^', '>', 'A'],
                vec!['>', '^', '^', 'A'],
                vec!['^', '>', '^', 'A'],
            ];
        }
        ('6', '0') => {
            return vec![
                vec!['v', 'v', '<', 'A'],
                vec!['<', 'v', 'v', 'A'],
                vec!['v', '<', 'v', 'A'],
            ];
        }
        ('0', '7') => {
            return vec![
                vec!['^', '^', '^', '<', 'A'],
                vec!['^', '^', '<', '^', 'A'],
                vec!['^', '<', '^', '^', 'A'],
            ];
        }
        ('7', '0') => {
            return vec![
                vec!['>', 'v', 'v', 'v', 'A'],
                vec!['v', 'v', '>', 'v', 'A'],
                vec!['v', '>', 'v', 'v', 'A'],
            ];
        }
        ('0', '8') => {
            return vec![vec!['^', '^', '^', 'A']];
        }
        ('8', '0') => {
            return vec![vec!['v', 'v', 'v', 'A']];
        }
        ('0', '9') => {
            return vec![
                vec!['^', '^', '^', '>', 'A'],
                vec!['^', '^', '>', '^', 'A'],
                vec!['^', '>', '^', '^', 'A'],
                vec!['>', '^', '^', '^', 'A'],
            ];
        }
        ('9', '0') => {
            return vec![
                vec!['v', 'v', 'v', '<', 'A'],
                vec!['v', 'v', '<', 'v', 'A'],
                vec!['v', '<', 'v', 'v', 'A'],
                vec!['<', 'v', 'v', 'v', 'A'],
            ];
        }
        ('1', 'A') => {
            return vec![vec!['>', '>', 'v', 'A'], vec!['>', 'v', '>', 'A']];
        }
        ('A', '1') => {
            return vec![vec!['^', '<', '<', 'A'], vec!['<', '^', '<', 'A']];
        }
        ('1', '2') => {
            return vec![vec!['>', 'A']];
        }
        ('2', '1') => {
            return vec![vec!['<', 'A']];
        }
        ('1', '3') => {
            return vec![vec!['>', '>', 'A']];
        }
        ('3', '1') => {
            return vec![vec!['<', '<', 'A']];
        }
        ('1', '4') => {
            return vec![vec!['^', 'A']];
        }
        ('4', '1') => {
            return vec![vec!['v', 'A']];
        }
        ('1', '5') => {
            return vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']];
        }
        ('5', '1') => {
            return vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']];
        }
        ('1', '6') => {
            return vec![
                vec!['^', '>', '>', 'A'],
                vec!['>', '^', '>', 'A'],
                vec!['>', '>', '^', 'A'],
            ];
        }
        ('6', '1') => {
            return vec![
                vec!['v', '<', '<', 'A'],
                vec!['<', 'v', '<', 'A'],
                vec!['<', '<', 'v', 'A'],
            ];
        }
        ('1', '7') => {
            return vec![vec!['^', '^', 'A']];
        }
        ('7', '1') => {
            return vec![vec!['v', 'v', 'A']];
        }
        ('1', '8') => {
            return vec![
                vec!['^', '^', '>', 'A'],
                vec!['^', '>', '^', 'A'],
                vec!['>', '^', '^', 'A'],
            ];
        }
        ('8', '1') => {
            return vec![
                vec!['v', 'v', '<', 'A'],
                vec!['v', '<', 'v', 'A'],
                vec!['<', 'v', 'v', 'A'],
            ];
        }
        ('1', '9') => {
            return vec![
                vec!['^', '^', '>', '>', 'A'],
                vec!['^', '>', '>', '^', 'A'],
                vec!['>', '>', '^', '^', 'A'],
                vec!['>', '^', '>', '^', 'A'],
                vec!['>', '^', '^', '>', 'A'],
                vec!['^', '>', '^', '>', 'A'],
            ]
        }
        ('9', '1') => {
            return vec![
                vec!['v', 'v', '<', '<', 'A'],
                vec!['v', '<', 'v', 'v', 'A'],
                vec!['v', '<', '<', 'v', 'A'],
                vec!['<', '<', 'v', 'v', 'A'],
                vec!['<', 'v', '<', 'v', 'A'],
                vec!['<', 'v', 'v', '<', 'A'],
            ]
        }
        ('2', 'A') => {
            return vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']];
        }
        ('A', '2') => {
            return vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']];
        }
        ('2', '3') => {
            return vec![vec!['>', 'A']];
        }
        ('3', '2') => {
            return vec![vec!['<', 'A']];
        }
        ('2', '4') => {
            return vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']];
        }
        ('4', '2') => {
            return vec![vec!['v', '>', 'A'], vec!['>', 'v', 'A']];
        }
        ('2', '5') => {
            return vec![vec!['^', 'A']];
        }
        ('5', '2') => {
            return vec![vec!['v', 'A']];
        }
        ('2', '6') => {
            return vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']];
        }
        ('6', '2') => {
            return vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']];
        }
        ('2', '7') => {
            return vec![
                vec!['^', '^', '<', 'A'],
                vec!['^', '<', '^', 'A'],
                vec!['<', '^', '^', 'A'],
            ];
        }
        ('7', '2') => {
            return vec![
                vec!['v', 'v', '>', 'A'],
                vec!['v', '>', 'v', 'A'],
                vec!['>', 'v', 'v', 'A'],
            ];
        }
        ('2', '8') => {
            return vec![vec!['^', '^', 'A']];
        }
        ('8', '2') => {
            return vec![vec!['v', 'v', 'A']];
        }
        ('2', '9') => {
            return vec![
                vec!['^', '^', '>', 'A'],
                vec!['^', '>', '^', 'A'],
                vec!['>', '^', '^', 'A'],
            ]
        }
        ('3', 'A') => {
            return vec![vec!['v', 'A']];
        }
        ('A', '3') => {
            return vec![vec!['^', 'A']];
        }
        ('3', '4') => {
            return vec![
                vec!['<', '<', '^', 'A'],
                vec!['<', '^', '<', 'A'],
                vec!['^', '<', '<', 'A'],
            ];
        }
        ('4', '3') => {
            return vec![
                vec!['>', '>', 'v', 'A'],
                vec!['>', 'v', '>', 'A'],
                vec!['v', '>', '>', 'A'],
            ];
        }
        ('3', '5') => {
            return vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']];
        }
        ('5', '3') => {
            return vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']];
        }
        ('3', '6') => {
            return vec![vec!['^', 'A']];
        }
        ('6', '3') => {
            return vec![vec!['v', 'A']];
        }
        ('3', '7') => {
            return vec![
                vec!['^', '^', '<', '<', 'A'],
                vec!['^', '<', '^', '<', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['<', '<', '^', '^', 'A'],
                vec!['<', '^', '<', '^', 'A'],
                vec!['<', '^', '^', '<', 'A'],
            ];
        }
        ('7', '3') => {
            return vec![
                vec!['v', 'v', '>', '>', 'A'],
                vec!['v', '>', 'v', '>', 'A'],
                vec!['v', '>', '>', 'v', 'A'],
                vec!['>', 'v', 'v', '>', 'A'],
                vec!['>', 'v', '>', 'v', 'A'],
                vec!['>', '>', 'v', 'v', 'A'],
            ];
        }
        ('3', '8') => {
            return vec![
                vec!['<', '^', '^', 'A'],
                vec!['^', '<', '^', 'A'],
                vec!['^', '^', '<', 'A'],
            ];
        }
        ('8', '3') => {
            return vec![
                vec!['>', 'v', 'v', 'A'],
                vec!['v', '>', 'v', 'A'],
                vec!['v', 'v', '>', 'A'],
            ];
        }
        ('3', '9') => {
            return vec![vec!['^', '^', 'A']];
        }
        ('9', '3') => {
            return vec![vec!['v', 'v', 'A']];
        }
        ('4', 'A') => {
            return vec![
                vec!['>', '>', 'v', 'v', 'A'],
                vec!['>', 'v', '>', 'v', 'A'],
                vec!['>', 'v', 'v', '>', 'A'],
                vec!['v', '>', '>', 'v', 'A'],
                vec!['v', '>', 'v', '>', 'A'],
            ];
        }
        ('A', '4') => {
            return vec![
                vec!['<', '^', '^', '<', 'A'],
                vec!['<', '^', '<', '^', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['^', '<', '^', '<', 'A'],
                vec!['^', '^', '<', '<', 'A'],
            ];
        }
        ('4', '5') => {
            return vec![vec!['>', 'A']];
        }
        ('5', '4') => {
            return vec![vec!['<', 'A']];
        }
        ('4', '6') => {
            return vec![vec!['>', '>', 'A']];
        }
        ('6', '4') => {
            return vec![vec!['<', '<', 'A']];
        }
        ('4', '7') => return vec![vec!['^', 'A']],
        ('7', '4') => return vec![vec!['v', 'A']],
        ('4', '8') => return vec![vec!['>', '^', 'A'], vec!['^', '>', 'A']],
        ('8', '4') => return vec![vec!['<', 'v', 'A'], vec!['v', '<', 'A']],
        ('4', '9') => {
            return vec![
                vec!['>', '>', '^', 'A'],
                vec!['>', '^', '>', 'A'],
                vec!['^', '>', '>', 'A'],
            ]
        }
        ('9', '4') => {
            return vec![
                vec!['<', '<', 'v', 'A'],
                vec!['<', 'v', '<', 'A'],
                vec!['v', '<', '<', 'A'],
            ]
        }
        ('5', 'A') => {
            return vec![
                vec!['>', 'v', 'v', 'A'],
                vec!['v', '>', 'v', 'A'],
                vec!['v', 'v', '>', 'A'],
            ];
        }
        ('A', '5') => {
            return vec![
                vec!['<', '^', '^', 'A'],
                vec!['^', '^', '<', 'A'],
                vec!['^', '<', '^', 'A'],
            ];
        }
        ('5', '6') => {
            return vec![vec!['>', 'A']];
        }
        ('6', '5') => {
            return vec![vec!['<', 'A']];
        }
        ('5', '7') => {
            return vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']];
        }
        ('7', '5') => {
            return vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']];
        }
        ('5', '8') => {
            return vec![vec!['^', 'A']];
        }
        ('8', '5') => {
            return vec![vec!['v', 'A']];
        }
        ('5', '9') => {
            return vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']];
        }
        ('9', '5') => {
            return vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']];
        }
        ('6', 'A') => {
            return vec![vec!['v', 'v', 'A']];
        }
        ('A', '6') => {
            return vec![vec!['^', '^', 'A']];
        }
        ('6', '7') => {
            return vec![
                vec!['<', '<', '^', 'A'],
                vec!['<', '^', '<', 'A'],
                vec!['^', '<', '<', 'A'],
            ];
        }
        ('7', '6') => {
            return vec![
                vec!['>', '>', 'v', 'A'],
                vec!['>', 'v', '>', 'A'],
                vec!['v', '>', '>', 'A'],
            ];
        }
        ('6', '8') => {
            return vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']];
        }
        ('8', '6') => {
            return vec![vec!['v', '>', 'A'], vec!['>', 'v', 'A']];
        }
        ('6', '9') => {
            return vec![vec!['^', 'A']];
        }
        ('9', '6') => {
            return vec![vec!['v', 'A']];
        }
        ('7', 'A') => {
            return vec![
                vec!['>', '>', 'v', 'v', 'v', 'A'],
                vec!['>', 'v', '>', 'v', 'v', 'A'],
                vec!['>', 'v', 'v', '>', 'v', 'A'],
                vec!['>', 'v', 'v', 'v', '>', 'A'],
                vec!['v', '>', '>', 'v', 'v', 'A'],
                vec!['v', '>', 'v', '>', 'v', 'A'],
                vec!['v', '>', 'v', 'v', '>', 'A'],
                vec!['v', 'v', '>', '>', 'v', 'A'],
                vec!['v', 'v', '>', 'v', '>', 'A'],
            ];
        }
        ('A', '7') => {
            return vec![
                vec!['^', '^', '^', '<', '<', 'A'],
                vec!['^', '^', '<', '^', '<', 'A'],
                vec!['^', '<', '^', '^', '<', 'A'],
                vec!['<', '^', '^', '^', '<', 'A'],
                vec!['^', '^', '<', '<', '^', 'A'],
                vec!['^', '<', '^', '<', '^', 'A'],
                vec!['<', '^', '^', '<', '^', 'A'],
                vec!['^', '<', '<', '^', '^', 'A'],
                vec!['<', '^', '<', '^', '^', 'A'],
            ];
        }
        ('7', '8') => return vec![vec!['>', 'A']],
        ('8', '7') => return vec![vec!['<', 'A']],
        ('7', '9') => return vec![vec!['>', '>', 'A']],
        ('9', '7') => return vec![vec!['<', '<', 'A']],
        ('8', 'A') => {
            return vec![
                vec!['>', 'v', 'v', 'v', 'A'],
                vec!['v', '>', 'v', 'v', 'A'],
                vec!['v', 'v', '>', 'v', 'A'],
                vec!['v', 'v', 'v', '>', 'A'],
            ];
        }
        ('A', '8') => {
            return vec![
                vec!['<', '^', '^', '^', 'A'],
                vec!['^', '<', '^', '^', 'A'],
                vec!['^', '^', '<', '^', 'A'],
                vec!['^', '^', '^', '<', 'A'],
            ];
        }
        ('8', '9') => return vec![vec!['>', 'A']],
        ('9', '8') => return vec![vec!['<', 'A']],
        ('9', 'A') => return vec![vec!['v', 'v', 'v', 'A']],
        ('A', '9') => return vec![vec!['^', '^', '^', 'A']],
        _ => {
            panic!("!!! {} -> {} ", t.0, t.1);
        }
    }
}
#[memoize]
fn buttons(t: char, s: char, d: i32, trace: bool) -> (i64, Vec<char>) {
    let mut reschars = Vec::new();
    let res: i64;
    if d == 0 {
        if trace {
            reschars.push(t);
        }
        return (1, reschars);
    }
    if s == t {
        if trace {
            reschars.push('A');
        }
        return (1, reschars);
    }
    match (s, t) {
        ('A', '^') => {
            //res.append(&mut buttons('<', 'A', d - 1));
            //res.append(&mut buttons('A', '<', d - 1));
            (res, reschars) = pickpath(vec![vec!['<', 'A']], d - 1, trace);
        }
        ('A', '>') => {
            //res.append(&mut buttons('v', 'A', d - 1));
            //res.append(&mut buttons('A', 'v', d - 1));
            (res, reschars) = pickpath(vec![vec!['v', 'A']], d - 1, trace);
        }
        ('A', 'v') => {
            (res, reschars) =
                pickpath(vec![vec!['<', 'v', 'A'], vec!['v', '<', 'A']], d - 1, trace);
            //v1.append(&mut buttons('v', '<', d - 1));
            //v1.append(&mut buttons('A', 'v', d - 1));
            //v2.append(&mut buttons('<', 'v', d - 1));
            //v2.append(&mut buttons('A', '<', d - 1));
        }
        ('A', '<') => {
            (res, reschars) = pickpath(
                vec![vec!['<', 'v', '<', 'A'], vec!['v', '<', '<', 'A']],
                d - 1,
                trace,
            );
        }
        ('^', 'A') => {
            (res, reschars) = pickpath(vec![vec!['>', 'A']], d - 1, trace);
        }
        ('^', '^') => return buttons('A', 'A', d - 1, trace),
        ('^', 'v') => return pickpath(vec![vec!['v', 'A']], d - 1, trace),

        ('^', '>') => {
            return pickpath(vec![vec!['>', 'v', 'A'], vec!['v', '>', 'A']], d - 1, trace)
        }

        ('^', '<') => {
            return pickpath(vec![vec!['v', '<', 'A']], d - 1, trace);
        }
        ('>', 'A') => {
            return pickpath(vec![vec!['^', 'A']], d - 1, trace);
        }
        ('>', '>') => return buttons('A', 'A', d - 1, trace),
        ('>', 'v') => return pickpath(vec![vec!['<', 'A']], d - 1, trace),
        ('>', '<') => return pickpath(vec![vec!['<', '<', 'A']], d - 1, trace),
        ('>', '^') => {
            return pickpath(vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']], d - 1, trace)
        }
        ('<', 'A') => {
            return pickpath(
                vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']],
                d - 1,
                trace,
            )
        }
        ('<', '<') => return buttons('A', 'A', d - 1, trace),
        ('<', 'v') => return pickpath(vec![vec!['>', 'A']], d - 1, trace),
        ('<', '^') => return pickpath(vec![vec!['>', '^', 'A']], d - 1, trace),
        ('<', '>') => return pickpath(vec![vec!['>', '>', 'A']], d - 1, trace),
        ('v', 'A') => {
            return pickpath(vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']], d - 1, trace)
        }
        ('v', 'v') => return buttons('A', 'A', d - 1, trace),
        ('v', '<') => return pickpath(vec![vec!['<', 'A']], d - 1, trace),
        ('v', '^') => return pickpath(vec![vec!['^', 'A']], d - 1, trace),
        ('v', '>') => return pickpath(vec![vec!['>', 'A']], d - 1, trace),
        t => {
            return pickpath(paths(t), d - 1, trace);
        }
    }

    return (res, reschars);
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut res = 0;
        for line in &self.lines {
            let base = line.as_str().trim_matches('A').parse::<i64>().unwrap();
            let c = testseq(line.to_string(), 3, true);
            res += (c as i64) * base;
        }
        return res;
    }
    fn metrics2(&self) -> i64 {
        let mut res = 0;
        for line in &self.lines {
            let base = line.as_str().trim_matches('A').parse::<i64>().unwrap();
            let c = testseq(line.to_string(), 26, false);
            res += (c as i64) * base;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 126384);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
