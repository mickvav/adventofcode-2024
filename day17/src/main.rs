use std::collections::HashMap;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Inp {
    registers: [i64; 4],
    program: Vec<i64>,
    ptr: usize,
    hm : HashMap<i64, i64>,

}

fn read_file(filename: &str) -> Inp {
    let regre = Regex::new(r"Register (A|B|C): ([0-9]+)").unwrap();
    let programre = Regex::new(r"Program: ([0-9,]+)").unwrap();
    let mut inp = Inp {
        registers: [0, 0, 0, 0],
        program: Vec::new(),
        ptr: 0,
        hm: HashMap::new(),
    };

    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in lines.lines() {
        if regre.is_match(line) { 
            let Some(caps) = regre.captures(line) else {todo!()};
            if &caps[1] == "A" {
                inp.registers[0] = caps[2].parse::<i64>().unwrap();
            }
            if &caps[1] == "B" {
                inp.registers[1] = caps[2].parse::<i64>().unwrap();
            }
            if &caps[1] == "C" {
                inp.registers[2] = caps[2].parse::<i64>().unwrap();
            }
        }
        if programre.is_match(line) {
            let Some(caps) = programre.captures(line) else {todo!()};
            for v in caps[1].split(",") {
                inp.program.push(v.parse::<i64>().unwrap());
            }
        }
    };
    return inp;
}



impl Inp {
    fn combo(&self) -> i64 {
        let v = self.program[self.ptr + 1];
        if v < 4 {
            return v;
        };
        if v == 4 {
            return self.registers[0];
        };
        if v == 5 {
            return self.registers[1];
        };
        if v == 6 {
            return self.registers[2];
        };
        panic!("unexpected input: {}", v);
    }

    fn adv(&mut self) {
        if self.registers[3] != 0 {
            println!("adv A: {:#016b} combo: {:#016b}", self.registers[0], self.combo());
        }
        let numerator = self.registers[0];
        self.registers[0] = numerator >> self.combo();
        self.ptr += 2;
    }

    fn bxl(&mut self) {
        if self.registers[3] != 0 {
            println!("bxl B: {:#016b} attr: {:#016b}", self.registers[1], self.program[self.ptr + 1]);
        }
        self.registers[1] = self.registers[1] ^ self.program[self.ptr + 1];
        self.ptr += 2;
    }

    fn bst(&mut self) {
        if self.registers[3] != 0 {
            println!(
                "bst combo: {:#016b}", self.combo()
                )
        }
        self.registers[1] = self.combo() & 0x7;
        self.ptr += 2;
    }

    fn jnz(&mut self) {
        if self.registers[0] != 0 {
            self.ptr = self.program[self.ptr + 1] as usize;
        } else {
            self.ptr += 2;
        }
    }

    fn bxc(&mut self) {
        if self.registers[3] != 0 {
            println!("bxc B:{:#016b} C:{:#016b}", self.registers[1], self.registers[2]);
        };
        self.registers[1] = self.registers[1] ^ self.registers[2];
        self.ptr += 2;
    }

    fn out(&mut self, res: &mut Vec<i64>) {
        if self.registers[3] != 0 {
            let ltb = self.registers[3] & 0x3FF; // lowest 10 bits of original A
            let v = self.combo() & 0x7;
            if self.hm.contains_key(&ltb) {
                if v != self.hm[&ltb] {
                    println!("A: {:#016b} B: {:#016b} C: {:#016b} D: {:#016b}", self.registers[0], self.registers[1], self.registers[2], self.registers[3]);
                    panic!("ups: {} {} {}", ltb, v, self.hm[&ltb]);
                }
            } else {
                self.hm.insert(ltb, v);
            }
            self.registers[3] = self.registers[3] >> 3;
            if self.registers[3] != self.registers[0] {
                panic!("2 ups: {} {}", self.registers[3], self.registers[0]);
            }
        }
        res.push(self.combo() & 0x7);
        self.ptr += 2;
    }

    fn bdv(&mut self){
        if self.registers[3] != 0 {
            println!("bdv: A:{:#016b} combo: {:#016b}", self.registers[0], self.combo())
        }
        let numerator = self.registers[0];
        self.registers[1] = numerator >> self.combo();
        self.ptr += 2;
    }

    fn cdv(&mut self) {
        if self.registers[3] != 0 {
            println!("cdv: A:{:#016b} combo: {:#016b}", self.registers[0], self.combo())
        }
        self.registers[2] = self.registers[0] >> self.combo();
        self.ptr += 2;
    }

    fn run(&mut self) -> Vec<i64> {
        let mut res: Vec<i64> = Vec::new();
        self.ptr = 0;
        while self.ptr < self.program.len() {
            match self.program[self.ptr] {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(&mut res),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => panic!("unexpected input: {}", self.program[self.ptr]),
            }
        }
        return res;
    }

    fn metrics(&mut self) -> String {
       let res = self.run();
       return res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    }
    fn runprog(&mut self, arg : i64) -> i64 {
        self.ptr = 0;
        self.registers[0] = arg;
        self.registers[1] = 0;
        self.registers[2] = 0;
        self.registers[3] = 0;
        let res = self.run();
        return self.vec2num(&res)   
    }

    fn vec2num(&self, v: &Vec<i64>) -> i64 {
        let mut out: i64 = 0;
        let mut base: i64 = 1;
        v.iter().for_each(|x| {
            out += base * x;
            base = base << 3;
        });
        return out;
    }
    fn findbits(&mut self, mask: i64, pos: i64, target: i64) -> (bool, i64) {
        let mut res = 0;
        for i in 0..8 {
            let j = mask | ((i as i64)<<pos);
            res = self.runprog(j);
            if (res & (0x7 << pos)) == ( target & (0x7 << pos)) {
                let g : bool;
                let recres: i64;
                if pos > 0 { 
                    (g, recres) = self.findbits(j, pos-3, target);
                    if g {
                        return (g, recres);
                    }
                } else {
                    return (res == target, j);
                }
            }
        }
        return (false, -1)
    }
    fn metrics2(&mut self) -> i64 {
        let target = self.vec2num(&self.program);
        let mut mask: i64 = 0;
        (_, mask) = self.findbits(0, 45, target);
        println!("target:{:o}", target);
        println!("mask  :{:o}", mask);
        println!("run   :{:o}", self.runprog(mask));
       return mask;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").run(), vec![4,6,3,5,6,3,5,2,1,0]);
        assert_eq!(read_file("test2.txt").metrics2(), 117440);
        
        let mut a = Inp{
                registers: [0,0,9, 0],
                program: [2,6].to_vec(),
                ptr: 0,
                hm: HashMap::new(),
            };
        a.run();
        assert_eq!(a.registers[1], 1);
    }
}

fn main() {
    let mut v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
