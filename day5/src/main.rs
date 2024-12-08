// use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Inp {
    rules: [i32; 10000],
    orders: Vec<Vec<usize>>,
}

fn read_file(filename: &str) -> Inp {
    let mut res = Inp {
        orders: vec![],
        rules: [0; 10000],
    };
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    contents.lines().for_each(|line| {
        if line.split("|").count() == 2 {
            let parts = line.split("|").collect::<Vec<&str>>();
            let a = parts[0].parse::<usize>().unwrap();
            let b = parts[1].parse::<usize>().unwrap();
            res.rules[a * 100 + b] = 1;
        }
        if line.split(",").count() > 1 {
            res.orders.push(
                line.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            );
        }
    });
    return res;
}

impl Inp {
    fn is_violation(&self, a: usize, b: usize) -> bool {
        return self.rules[b * 100 + a] == 1;
    }
    fn find_violation(&self, order: &Vec<usize>) -> (bool, usize, usize) {
        for i in 0..order.len() {
            for j in 0..i {
                if self.is_violation(order[j], order[i]) {
                    return (true, j, i);
                }
            }
        }
        return (false, 0, 0);
    }
    fn reorder(&self, j:usize, i:usize,order: &Vec<usize>) -> Vec<usize> {
        let mut res = vec![];
        res.append(&mut order[0..j].to_vec());
        res.append(&mut order[i..i+1].to_vec());
        res.append(&mut order[j..i].to_vec());
        res.append(&mut order[i+1..order.len()].to_vec());
        return res;
    }
    fn reorderFull(&self, j:usize, i:usize, order: &Vec<usize>) -> Vec<usize> {
        let mut need = true;
        let mut jr = j;
        let mut ir = i;
        let mut res = order.clone();
        while need {
            res = self.reorder(jr, ir, &res);
            (need, jr, ir) = self.find_violation(&res);
        }
        return res;
    }

    fn metrics(&self) -> usize {
        let mut res = 0;
        for order in &self.orders {
            let mut is_violation = false;
            'outer: for i in 0..order.len() {
                for j in 0..i {
                    if self.is_violation(order[j], order[i]) {
                        is_violation = true;
                        break 'outer;
                    }
                }
            }
            if !is_violation {
                res += order[order.len() / 2];
            }
        }
        return res;
    }
    fn metrics2(&self) -> usize {
        let mut res = 0;
        for order in &self.orders {
            let (v, j, i) = self.find_violation(order);
            if v {
                let new_order = self.reorderFull(j, i, order);
                res += new_order[new_order.len() / 2];
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
        assert_eq!(read_file("test.txt").metrics(), 143);
        assert_eq!(read_file("test.txt").metrics2(), 123);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
