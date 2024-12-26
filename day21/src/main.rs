use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Inp {
    targets: Vec<String>,
}

#[derive(Debug)]
struct Spect {
    s: HashMap<String, i64>
}

impl Spect {
    fn add(&mut self, s: Spect) {
        s.s.iter().for_each(|x| {
            if self.s.contains_key(x.0) {
                self.s.insert(x.0.to_string(), self.s[x.0] + x.1);
            } else {
                self.s.insert(x.0.to_string(), *x.1);
            }
        })
    }
    fn addmult(&mut self, s: Spect, alpha: i64) {
        s.s.iter().for_each(|x| {
            if self.s.contains_key(x.0) {
                self.s.insert(x.0.to_string(), self.s[x.0] + x.1 * alpha);
            } else {
                self.s.insert(x.0.to_string(), *x.1 * alpha);
            }
        })
    }
    fn complexity(&self) -> i64 {
        let mut res = 0;
        self.s.iter().for_each(|x| {
            res += x.1 * x.0.len() as i64;
        });
        return res;
    }
    fn mutate(&self) -> Spect {
        let mut res = Spect { s: HashMap::new() };
        let mut h = HashMap::new();
        self.s.iter().for_each(|x| {
            let new = direct_mid(x.0.clone(), &mut h);
            let s1 = spect_from_string(new);
            res.addmult(s1, *x.1);
        });
        return res;
    }
}

fn spect_from_string(s: String) -> Spect {
    let mut res = Spect { s: HashMap::new() };
    s.split("A").for_each(|x| {
        let mut xs = x.to_string();
        xs.push_str("A");
        if res.s.contains_key(&xs) {
            res.s.insert(xs.clone(), res.s[&xs] + 1);
        } else {
            res.s.insert(xs.clone(), 1);
        }
    });
    if s.ends_with("A") {
        if res.s.contains_key("A") {
            res.s.insert("A".to_string(), res.s["A"] - 1);
        }
    }
    return res;
}





fn pick_optimal(s: &String, h: &mut HashMap<String, String>) -> String {
    let mut res = String::new();
    let mut length = 100000;
    let mut iters = 0;
    s.split(",").for_each(|x| {
        let l = direct_first(direct_mid(x.to_string(), h)).len();
        if l < length {
            iters = iters + 1;
            length = l;
            res = x.to_string();
        }
    });
    if iters > 1 {
        println!("!! {} {}", s, res);
    }
    return res;
}

fn pick_optimal2(s: &String, h: &mut HashMap<String, String>) -> String {
    if h.contains_key(s) {
        return h[s].to_string();
    }
    let mut res = String::new();
    let mut length = 100000;
    let mut iters = 0;

    s.split(",").for_each(|x| {
        let l = direct_first(direct_first(x.to_string())).len();
        if l < length {
            iters = iters + 1;
            println!("== l: {} length: {} res: {} x: {} ", l, length, res, x);
            length = l;
            res = x.to_string();
        }
    });
    if iters > 1 {
        println!("!! {} {}", s, res);
    }
    h.insert(s.to_string(), res.to_string());
    return res;
}

fn direct_last(s: String, h: &mut HashMap<String, String>) -> String {
    let mut res = String::new();
    let mut pos = 'A';
    let shortest: HashMap<(char, char), String> = HashMap::from([
        (('A', '0'), "<A".to_string()),
        (('A', '1'), "^<<A,<^<A".to_string()),
        (('A', '2'), "^<A,<^A".to_string()),
        (('A', '3'), "^A".to_string()),
        (('A', '4'), "^^<<A,<^^<A,<^<^A,^<^<A".to_string()),
        (('A', '5'), "^^<A,^^<A,<^^A".to_string()),
        (('A', '6'), "^^A".to_string()),
        (
            ('A', '7'),
            "^^^<<A,^^<^<A,^^<<^A,^<^<^A,^<<^^A,<^^<^A,<^^^<A,<^<^^A".to_string(),
        ),
        (('A', '8'), "^^^<A,<^^^A,^<^^A,^^<^A".to_string()),
        (('A', '9'), "^^^A".to_string()),
        (('0', '0'), "A".to_string()),
        (('0', '1'), "^<A".to_string()),
        (('0', '2'), "^A".to_string()),
        (('0', '3'), "^>A,>^A".to_string()),
        (('0', '4'), "^^<A,^<^A".to_string()),
        (('0', '5'), "^^A".to_string()),
        (('0', '6'), "^^>A,>^^A,^>^A".to_string()),
        (('0', '7'), "^^^<A,^^<^A,^<^^A".to_string()),
        (('0', '8'), "^^^A".to_string()),
        (('0', '9'), "^^^>A,^^>^A,^>^^A,>^^^A".to_string()),
        (('0', 'A'), ">A".to_string()),
        (('1', '0'), ">vA".to_string()),
        (('1', '1'), "A".to_string()),
        (('1', '2'), ">A".to_string()),
        (('1', '3'), ">>A".to_string()),
        (('1', '4'), "^A".to_string()),
        (('1', '5'), "^>A,>^A".to_string()),
        (('1', '6'), "^>>A,>^>A,>>^A".to_string()),
        (('1', '7'), "^^A".to_string()),
        (('1', '8'), "^^>A,^>^A,>^^A".to_string()),
        (
            ('1', '9'),
            "^^>>A,>>^^A,^>^>A,>^>^A,>^^>A,^>>^A".to_string(),
        ),
        (('1', 'A'), ">>vA,>v>A".to_string()),
        (('2', '0'), "vA".to_string()),
        (('2', '1'), "<A".to_string()),
        (('2', '2'), "A".to_string()),
        (('2', '3'), ">A".to_string()),
        (('2', '4'), "<^A,^<A".to_string()),
        (('2', '5'), "^A".to_string()),
        (('2', '6'), "^>A,>^A".to_string()),
        (('2', '7'), "<^^A,^^<A,^<^A".to_string()),
        (('2', '8'), "^^A".to_string()),
        (('2', '9'), ">^^A,^>^A,^^>A".to_string()),
        (('2', 'A'), ">vA,v>A".to_string()),
        (('3', '0'), "<vA,v<A".to_string()),
        (('3', '1'), "<<A".to_string()),
        (('3', '2'), "<A".to_string()),
        (('3', '3'), "A".to_string()),
        (('3', '4'), "^<<A,<<^A,<^<A".to_string()),
        (('3', '5'), "^<A,<^A".to_string()),
        (('3', '6'), "^A".to_string()),
        (
            ('3', '7'),
            "<<^^A,<^^<A,<^<^A,^<^<A,^<<^A,^^<<A".to_string(),
        ),
        (('3', '8'), "^^<A,<^^A,^<^A".to_string()),
        (('3', '9'), "^^A".to_string()),
        (('3', 'A'), "vA".to_string()),
        (('4', '0'), ">vvA,v>vA".to_string()),
        (('4', '1'), "vA".to_string()),
        (('4', '2'), ">vA,v>A".to_string()),
        (('4', '3'), ">>vA,>v>A,v>>A".to_string()),
        (('4', '4'), "A".to_string()),
        (('4', '5'), ">A".to_string()),
        (('4', '6'), ">>A".to_string()),
        (('4', '7'), "^A".to_string()),
        (('4', '8'), "^>A,>^A".to_string()),
        (('4', '9'), "^>>A,>^>A,>>^A".to_string()),
        (('4', 'A'), ">>vvA,>v>vA,>vv>A,v>>vA,v>v>A".to_string()),
        (('5', '0'), "vvA".to_string()),
        (('5', '1'), "v<A,<vA".to_string()),
        (('5', '2'), "vA".to_string()),
        (('5', '3'), "v>A,>vA".to_string()),
        (('5', '4'), "<A".to_string()),
        (('5', '5'), "A".to_string()),
        (('5', '6'), ">A".to_string()),
        (('5', '7'), "^<A,<^A".to_string()),
        (('5', '8'), "^A".to_string()),
        (('5', '9'), "^>A,>^A".to_string()),
        (('5', 'A'), ">vvA,v>vA,vv>A".to_string()),
        (('6', '0'), "<vvA,vv<A,v<vA".to_string()),
        (('6', '1'), "<<vA,<v<A,v<<A".to_string()),
        (('6', '2'), "<vA,v<A".to_string()),
        (('6', '3'), "vA".to_string()),
        (('6', '4'), "<<A".to_string()),
        (('6', '5'), "<A".to_string()),
        (('6', '6'), "A".to_string()),
        (('6', '7'), "^<<A,<<^A,<^<A".to_string()),
        (('6', '8'), "^<A,^<A".to_string()),
        (('6', '9'), "^A".to_string()),
        (('6', 'A'), "vvA".to_string()),
        (('7', '0'), ">vvvA,v>vvA,vv>vA".to_string()),
        (('7', '1'), "vvA".to_string()),
        (('7', '2'), "vv>A,v>vA,>vvA".to_string()),
        (
            ('7', '3'),
            "vv>>A,>>vvA,>v>vA,v>>vA,v>v>A,>vv>A,v>>vA".to_string(),
        ),
        (('7', '4'), "vA".to_string()),
        (('7', '5'), "v>A,>vA".to_string()),
        (('7', '6'), "v>>A,>>vA,>v>A".to_string()),
        (('7', '7'), "A".to_string()),
        (('7', '8'), ">A".to_string()),
        (('7', '9'), ">>A".to_string()),
        (
            ('7', 'A'),
            ">>vvvA,>v>vvA,>vv>vA,>vvv>A,v>>vvA,v>v>vA,v>vv>A,vv>v>A,vv>>vA".to_string(),
        ),
        (('8', '0'), "vvvA".to_string()),
        (('8', '1'), "vv<A,v<vA,<vvA".to_string()),
        (('8', '2'), "vvA".to_string()),
        (('8', '3'), "vv>A,v>vA,>vvA".to_string()),
        (('8', '4'), "v<A,<vA".to_string()),
        (('8', '5'), "vA".to_string()),
        (('8', '6'), "v>A,>vA".to_string()),
        (('8', '7'), "<A".to_string()),
        (('8', '8'), "A".to_string()),
        (('8', '9'), ">A".to_string()),
        (('8', 'A'), "vvvA".to_string()),
        (('9', '0'), "vvv<A,vv<vA,v<vvA,<vvvA".to_string()),
        (
            ('9', '1'),
            "vv<<A,v<<vA,v<v<A,<<vvA,<v<vA,<vv<A".to_string(),
        ),
        (('9', '2'), "vv<A,v<vA,<vvA".to_string()),
        (('9', '3'), "vvA".to_string()),
        (('9', '4'), "v<<A,<v<A,<<vA".to_string()),
        (('9', '5'), "v<A,<vA".to_string()),
        (('9', '6'), "vA".to_string()),
        (('9', '7'), "<<A".to_string()),
        (('9', '8'), "<A".to_string()),
        (('9', '9'), "A".to_string()),
        (('9', 'A'), "vvvA".to_string()),
    ]);
    for c in s.chars() {
        let tpl = (pos, c);
        if !shortest.contains_key(&tpl) {
            println!("Key not fount: {} {}", pos, c);
        };
        res.push_str(pick_optimal(&shortest[&tpl], h).as_str());
        pos = c;
    }
    return res;
}

fn direct_first(s: String) -> String {
    let mut res = String::new();
    let mut pos = 'A';
    let shortest: HashMap<(char, char), String> = HashMap::from([
        (('A', 'A'), "A".to_string()),
        (('A', '<'), "<v<A".to_string()),
        (('A', '^'), "<A".to_string()),
        (('A', '>'), "vA".to_string()),
        (('A', 'v'), "<vA".to_string()),
        (('<', 'A'), ">>^A".to_string()),
        (('<', '<'), "A".to_string()),
        (('<', '^'), ">^A".to_string()),
        (('<', '>'), ">>A".to_string()),
        (('<', 'v'), ">A".to_string()),
        (('^', 'A'), ">A".to_string()),
        (('^', '<'), "v<A".to_string()),
        (('^', '^'), "A".to_string()),
        (('^', '>'), "v>A".to_string()),
        (('^', 'v'), "vA".to_string()),
        (('>', 'A'), "^A".to_string()),
        (('>', '<'), "<<A".to_string()),
        (('>', '^'), "<^A".to_string()),
        (('>', '>'), "A".to_string()),
        (('>', 'v'), "<A".to_string()),
        (('v', 'A'), ">^A".to_string()),
        (('v', '<'), "<A".to_string()),
        (('v', '^'), "^A".to_string()),
        (('v', '>'), ">A".to_string()),
        (('v', 'v'), "A".to_string()),
    ]);
    for c in s.chars() {
        let tpl = (pos, c);
        res.push_str(&shortest[&tpl]);
        pos = c;
    }
    return res;
}

fn direct_mid2hashed(
    s: String,
    h: &mut HashMap<String, String>,
    h1: &mut HashMap<String, String>,
    g: usize,
) -> String {
    let mut res = String::new();
    let mut calls = 0;
    let mut xs = String::new();
    let mut i = 0;
    s.split("A").for_each(|x| {
        xs.push_str(x);
        xs.push_str("A");
        if i < g {
            i += 1;
            return;
        } else {
            if h1.contains_key(&xs) {
                res.push_str(&h1[&xs]);
            } else {
                let v = direct_mid(xs.clone(), h);
                calls += 1;
                res.push_str(v.as_str());
                h1.insert(xs.clone(), v);
            }
            xs.clear();
            i = 0;
        }
    });
    if h1.contains_key(&xs) {
        res.push_str(&h1[&xs]);
    } else {
        let v = direct_mid(xs.clone(), h);
        calls += 1;
        res.push_str(v.as_str());
        h1.insert(xs.clone(), v);
    }
    xs.clear();

    println!("c: {}", calls);
    res.pop();
    return res;
}
fn direct_mid(s: String, h: &mut HashMap<String, String>) -> String {
    let mut res = String::new();
    let mut pos = 'A';
    let shortest: HashMap<(char, char), String> = HashMap::from([
        (('A', 'A'), "A".to_string()),
        (('A', '<'), "<v<A,v<<A".to_string()),
        (('A', '^'), "<A".to_string()),
        (('A', '>'), "vA".to_string()),
        (('A', 'v'), "<vA,v<A".to_string()),
        (('<', 'A'), ">>^A,>^>A".to_string()),
        (('<', '<'), "A".to_string()),
        (('<', '^'), ">^A".to_string()),
        (('<', '>'), ">>A".to_string()),
        (('<', 'v'), ">A".to_string()),
        (('^', 'A'), ">A".to_string()),
        (('^', '<'), "v<A".to_string()),
        (('^', '^'), "A".to_string()),
        (('^', '>'), "v>A,>vA".to_string()),
        (('^', 'v'), "vA".to_string()),
        (('>', 'A'), "^A".to_string()),
        (('>', '<'), "<<A".to_string()),
        (('>', '^'), "<^A,^<A".to_string()),
        (('>', '>'), "A".to_string()),
        (('>', 'v'), "<A".to_string()),
        (('v', 'A'), ">^A,^>A".to_string()),
        (('v', '<'), "<A".to_string()),
        (('v', '^'), "^A".to_string()),
        (('v', '>'), ">A".to_string()),
        (('v', 'v'), "A".to_string()),
    ]);
    for c in s.chars() {
        let tpl = (pos, c);
        res.push_str(pick_optimal2(&shortest[&tpl], h).as_str());
        pos = c;
    }
    return res;
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = buf.lines();
    let mut res = Inp {
        targets: Vec::new(),
    };

    for line in lines {
        res.targets.push(line.to_string());
    }
    return res;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut res = 0;
        let mut h = HashMap::new();
        for line in &self.targets {
            let base = line.as_str().trim_matches('A').parse::<i64>().unwrap();
            let v = direct_mid(
                direct_mid(direct_last(line.to_string(), &mut h), &mut h),
                &mut h,
            )
            .len();
            println!("{} * {}", base, v);
            res += (v as i64) * base;
        }
        return res;
    }
    fn metrics2(&self, l: usize) -> i64 {
        let mut res = 0;
        let mut h = HashMap::new();
//      let mut h1 = HashMap::new();
        for line in &self.targets {
            let base = line.as_str().trim_matches('A').parse::<i64>().unwrap();
            let mut g = direct_last(line.to_string(), &mut h);
            println!("{}", g);
            let start = Instant::now();
            let mut s = spect_from_string(g.clone());
            println!("S: {}", s.complexity());
            println!("s: {:?}", s);
            println!("g: {:?}", g);
            for i in 0..l {
   //             println!("{}    {}", i, g.len());
                //      println!("... {}", g);
                //  let g0 = g.clone();
                //g = direct_mid2hashed(g, &mut h, &mut h1, 32);
                s = s.mutate();

                println!("S.c: {}", s.complexity());
                println!("s: {:?}", s);
                // g = direct_mid(g, &mut h);
                println!("{:?}", start.elapsed());
                //   let g1= direct_mid(g0, &mut h);
                //     println!(">>> {}", g);
                //   println!(">>> {}", g1);
            }
            let v = s.complexity();
            println!("{} * {}", base, v);
            res += (v as i64) * base;
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
        assert_eq!(read_file("test.txt").metrics2(2), 126384);
    }

    #[test]
    fn simple() {
        let mut h = HashMap::new();
        assert_eq!(
            direct_last("029A".to_string(), &mut h),
            "<A^A>^^AvvvA".to_string()
        );
        assert_eq!(
            direct_mid(direct_last("029A".to_string(), &mut h), &mut h),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string()
        );
        assert_eq!(pick_optimal2(&"<v<A,v<<A".to_string(), &mut h), "<v<A");

        /*     assert_eq!(direct_mid(direct_mid(direct_last("029A".to_string()))),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string()); */
        assert_eq!(direct_last("379A".to_string(), &mut h), "^A<<^^A>>AvvvA");
        assert_eq!(
            direct_mid(direct_last("379A".to_string(), &mut h), &mut h),
            "<A>Av<<AA>^AA>AvAA^A<vAAA>^A"
        );
        println!("<A>A<AAv<AA>>^AvAA^A<vAAA>^A");
           assert_eq!(direct_mid(
            direct_mid(
                direct_last("379A".to_string(), &mut h),
                &mut h,
            ),
            &mut h,
        )
        ,"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2(25));
}
