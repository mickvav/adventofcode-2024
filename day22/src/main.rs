use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    seeds: Vec<i64>,
    values: Vec<HashMap<i64, i64>>,
}

fn get_values(seed: i64) -> HashMap<i64, i64> {
    let mut res = HashMap::new();
    let mut secret = seed;
    let mut pricekey: i64 = seed % 10;
    let mut deltakey: i64 = 0;
    for i in 0..2000 {
        secret = next(secret);
        let price = secret % 10;
        let prevprice = pricekey & 0xFF;
        pricekey = ((pricekey << 8) + price) & 0xFFFFFFFFFF;
        let delta = price - prevprice;
        deltakey = ((deltakey << 8) + 10 + delta) & 0xFFFFFFFF;
        if i < 3 {
            continue;
        }
        if res.contains_key(&deltakey) {
            continue;
        }
        res.insert(deltakey, price);
    }
    return res;
}

fn deltakey_to_string(deltakey: i64) -> String {
    let mut res = String::new();
    let mut dk = deltakey;
    while dk > 0 {
        let d = (dk & 0xFF) - 10;
        res = <i64>::to_string(&d) + "," + &res;
        dk = dk >> 8;
    }
    return res;
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = buf.lines();
    let mut res = Inp {
        seeds: Vec::new(),
        values: Vec::new(),
    };

    for line in lines {
        let seed = line.parse::<i64>().unwrap();
        res.seeds.push(seed);
        res.values.push(get_values(seed));
    }
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
    let s1 = prune(mix(s, secret));
    let s2 = prune(mix(s1 >> 5, s1)); // /32
    let s3 = prune(mix(s2 << 11, s2)); // *2048
    return s3;
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
        let mut hashdeltakeys: HashMap<i64, bool> = HashMap::new();
        for i in 0..self.seeds.len() {
            for hm in self.values[i].iter() {
                hashdeltakeys.insert(*hm.0, true);
            }
        }
        let mut res: i64 = 0;
        for dk in hashdeltakeys.keys() {
            let mut v: i64 = 0;
            for i in 0..self.seeds.len() {
                if self.values[i].contains_key(dk) {
                    v += self.values[i][dk];
                }
            }
            if v > res {
                res = v;
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
        assert_eq!(read_file("test.txt").metrics(), 37327623);
        assert_eq!(read_file("test2.txt").metrics2(), 23);
    }

    #[test]
    fn simple() {
        assert_eq!(next(123), 15887950);
        assert_eq!(deltakey_to_string(0x0a0a0a0b), "0,0,0,1,")
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
