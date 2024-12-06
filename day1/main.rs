use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
struct Inp{
    left : Vec<i32>,
    right : Vec<i32>
}

fn read_file(filename: &str ) -> Inp {

    let mut res =    Inp{ left: vec![], right: vec![]};
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents.lines().for_each(|line| {
        let mut parts = line.split("   ");
        let left = parts.next().unwrap().parse::<i32>().unwrap();
        let right = parts.next().unwrap().parse::<i32>().unwrap();
        res.left.push(left);
        res.right.push(right);
    });
    return res
}

impl Inp {
    fn metrics(&self) -> i32 {
        let mut res = 0;
        for i in 0..self.left.len() {
            res += (self.left[i] - self.right[i]).abs();
        }
        return res
    }
    fn metrics2(&self) -> i32 {
        let mut res: i32 = 0;
        let mut freqs: HashMap<i32, i32> = HashMap::new();

        for v in &self.right {
            freqs.entry(*v).and_modify(|e| *e += 1).or_insert(1);
        }
        for i in &self.left {
            res += i*freqs.get(i).unwrap_or(&0);
        }

        return res
    }
}

fn main() {
    let mut v = read_file("input.txt");
    v.left.sort();
    v.right.sort();
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());

}
