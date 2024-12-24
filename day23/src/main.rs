use std::collections::HashMap;
use std::fs;

type Node = (char, char);
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Triple {
    a: Node,
    b: Node,
    c: Node,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Nple {
    nodes: Vec<Node>,
}

impl Triple {
    fn starts_with_t(&self) -> bool {
        return self.a.0 == 't' || self.b.0 == 't' || self.c.0 == 't';
    }
}

fn get_nple(s: Vec<Node>) -> Nple {
    let mut r = s.clone();
    r.sort();
    return Nple { nodes: r };
}

impl Nple {
    fn extend(&self, n: Node) -> Nple {
        let mut r = self.nodes.clone();
        r.push(n);
        r.sort();
        return Nple { nodes: r };
    }
    fn password(&self) -> String {
        let mut res = String::new();
        for n in self.nodes.iter() {
            res.push(n.0);
            res.push(n.1);
            res.push(',');
        }
        return res.trim_end_matches(',').to_string();
    }
}

fn to_nple(s: Triple) -> Nple {
    return get_nple(vec![s.a, s.b, s.c]);
}
fn get_triple(s1: Node, s2: Node, s3: Node) -> Triple {
    if s1 < s2 {
        if s2 < s3 {
            return Triple {
                a: s1,
                b: s2,
                c: s3,
            };
        } else {
            // s3 < s2
            if s1 < s3 {
                return Triple {
                    a: s1,
                    b: s3,
                    c: s2,
                };
            } else {
                return Triple {
                    a: s3,
                    b: s1,
                    c: s2,
                };
            }
        }
    } else {
        // s2 < s1
        if s1 < s3 {
            return Triple {
                a: s2,
                b: s1,
                c: s3,
            };
        } else {
            // s3 < s1
            if s2 < s3 {
                return Triple {
                    a: s2,
                    b: s3,
                    c: s1,
                };
            } else {
                return Triple {
                    a: s3,
                    b: s2,
                    c: s1,
                };
            }
        }
    }
}

#[derive(Debug)]
struct Inp {
    nodes: HashMap<Node, bool>,
    linked: HashMap<Node, HashMap<Node, bool>>,
}

fn read_file(filename: &str) -> Inp {
    let buf = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = buf.lines();
    let mut res = Inp {
        nodes: HashMap::new(),
        linked: HashMap::new(),
    };

    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        if parts.len() != 2 {
            continue;
        }
        let a: Vec<char> = parts[0].chars().collect();
        let b: Vec<char> = parts[1].chars().collect();
        if a.len() != 2 || b.len() != 2 {
            continue;
        };
        let k1: Node = (a[0], a[1]);
        let k2: Node = (b[0], b[1]);
        res.linked
            .entry(k1)
            .and_modify(|e| {
                println!("{:?} -> {:?}", k1, k2);
                e.insert(k2, true);
            })
            .or_insert_with(|| {
                let mut h = HashMap::new();
                h.insert(k2, true);
                h
            });
        res.linked
            .entry(k2)
            .and_modify(|e| {
                println!("{:?} -> {:?}", k2, k1);
                e.insert(k1, true);
            })
            .or_insert_with(|| {
                let mut h = HashMap::new();
                h.insert(k1, true);
                h
            });
        //res.linked.insert(HashMap::new());
        res.nodes.insert(k1, true);
        res.nodes.insert(k2, true);
    }
    println!("{:?}", res);
    return res;
}

impl Inp {
    fn is_node_fully_linked(&self, n: Node, m: Nple) -> bool {
        for k in m.nodes {
            if !self.linked[&n].contains_key(&k) {
                return false;
            }
        }
        return true;
    }

    fn all_nple_links(&self, m: Nple) -> Vec<Node> {
        let mut res: Vec<Node> = Vec::new();
        let mut res_hash: HashMap<Node, bool> = HashMap::new();
        for n in m.nodes {
            for nbr in self.linked[&n].keys() {
                res_hash.insert(*nbr, true);
            }
        }
        for k in res_hash.keys() {
            res.push(k.clone());
        }
        return res;
    }
    fn max_nple(&self, m: Nple, rvhash: &mut HashMap<Nple, Nple>) -> Nple {
        if rvhash.contains_key(&m) {
            return rvhash[&m].clone();
        }
        let all_nbrs = self.all_nple_links(m.clone());
        let mut candidates: Vec<Nple> = Vec::new();
        for nbr in all_nbrs {
            if self.is_node_fully_linked(nbr, m.clone()) {
                candidates.push(self.max_nple(m.extend(nbr), rvhash));
            }
        }
        if candidates.len() == 0 {
            rvhash.insert(m.clone(), m.clone());
            return m;
        } else {
            let mut best = candidates[0].clone();
            for c in candidates {
                if c.nodes.len() > best.nodes.len() {
                    best = c;
                }
            }
            rvhash.insert(m.clone(), best.clone());
            return best;
        }
    }
    fn triplets(&self) -> Vec<Triple> {
        let mut res: Vec<Triple> = Vec::new();
        let mut res_hash: HashMap<Triple, bool> = HashMap::new();
        for a in self.nodes.keys() {
            for b in self.linked[a].keys() {
                for c in self.linked[b].keys() {
                    if c != a && c != b {
                        if self.linked[c].contains_key(a) {
                            println!("T: {:?} {:?} {:?}", a, b, c);
                            let t = get_triple(*a, *b, *c);
                            println!("T: {:?}", t);
                            res_hash.insert(t, true);
                        }
                    }
                }
            }
        }
        for k in res_hash.keys() {
            println!("{:?}", k);
            res.push(k.clone());
        }
        return res;
    }

    fn metrics(&self) -> i32 {
        let mut res = 0;
        let t = self.triplets();
        for i in 0..t.len() {
            if t[i].starts_with_t() {
                res += 1;
            }
        }
        return res;
    }
    fn metrics2(&self) -> String {
        let t = self.triplets();
        let mut rvhash: HashMap<Nple, Nple> = HashMap::new();
        let mut best = Nple { nodes: Vec::new() };
        for i in 0..t.len() {
            let n = to_nple(t[i].clone());
            let c = self.max_nple(n, &mut rvhash);
            if c.nodes.len() > best.nodes.len() {
                best = c;
            }
        }
        return best.password();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 7);
        assert_eq!(read_file("test.txt").metrics2(), "co,de,ka,ta")
    }

    #[test]
    fn order() {
        assert_eq!(
            get_triple(('q', 'p'), ('t', 'd'), ('w', 'h')),
            get_triple(('t', 'd'), ('q', 'p'), ('w', 'h')),
        );
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
