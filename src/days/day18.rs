use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };
use std::thread;
use std::time::Duration;

pub struct Day18;

impl ChristmasDay for Day18 {
    fn solve_a(&self, data: &str) -> String {
        let (atx, arx) = mpsc::channel::<i64>();
        run_prog(data, (atx, arx), 0, ProblemPart::A)
    }
    fn solve_b(&self, data: &str) -> String {
        let (atx, arx) = mpsc::channel::<i64>();
        let (btx, brx) = mpsc::channel::<i64>();
        let adata = data.to_string().clone();
        let t1 = thread::spawn(move || {
            run_prog(&adata, (btx, arx), 0, ProblemPart::B);
        });
        let bdata = data.to_string().clone();
        let t2 = thread::spawn(move || -> String {
            run_prog(&bdata, (atx, brx), 1, ProblemPart::B)
        });

        t1.join().unwrap();
        t2.join().unwrap().to_string()
    }

}

fn run_prog(data: &str, (snd, rcv): (Sender<i64>, Receiver<i64>), pid: i64, prob: ProblemPart) -> String {
    let mut registers: HashMap<String, i64> = HashMap::new();
    if prob == ProblemPart::B {
        registers.insert("p".to_string(), pid);
    }
    let lines: Vec<&str> = data.lines().collect::<_>();
    let mut line = 0;
    let mut ret = 0;
    while line < lines.len() {
        let istr = lines[line].split(" ").collect::<Vec<&str>>();
        match istr[0] {
            "snd" => {
                if prob == ProblemPart::B && pid == 1 {
                    ret += 1;
                }
                let val: i64 = match registers.get(istr[1]) {
                    Some(&sound) => {
                        sound
                    },
                    None => istr[1].parse::<i64>().unwrap(),
                };
                snd.send(val).unwrap();
            },
            "set" => {
                let val = match registers.get(istr[2]) {
                    Some(&v) => v,
                    None => istr[2].parse::<i64>().unwrap()
                };
                registers.insert(istr[1].to_string(), val);
            },
            "add" => {
                let val = match registers.get(istr[2]) {
                    Some(&v) => v,
                    None => istr[2].parse::<i64>().unwrap()
                };
                let inc = registers.entry(istr[1].to_string()).or_insert(0);
                *inc += val;
            },
            "mul" => {
                let val = match registers.get(istr[2]) {
                    Some(&v) => v,
                    None => istr[2].parse::<i64>().unwrap()
                };
                let inc = registers.entry(istr[1].to_string()).or_insert(0);
                *inc *= val;
            },
            "mod" => {
                let val = match registers.get(istr[2]) {
                    Some(&v) => v,
                    None => istr[2].parse::<i64>().unwrap()
                };
                let inc = registers.entry(istr[1].to_string()).or_insert(0);
                *inc = *inc % val;
            },
            "rcv" => {
                match prob {
                    ProblemPart::A => {
                        let val = match registers.get(istr[1]) {
                            Some(&v) => v,
                            None => istr[1].parse::<i64>().unwrap()
                        };
                        if val != 0 {
                            while let Ok(x) = rcv.try_recv() {
                                ret = x;
                            }
                            break;
                        }
                    },
                    ProblemPart::B => {
                        match rcv.recv_timeout(Duration::from_millis(1000)) {
                            Ok(val) => registers.insert(istr[1].to_string(), val),
                            Err(_) => {
                                break;
                            },
                        };
                    },
                }
            },
            "jgz" => {
                let valx = match registers.get(istr[1]) {
                    Some(&v) => v,
                    None => istr[1].parse::<i64>().unwrap()
                };
                let valy = match registers.get(istr[2]) {
                    Some(&v) => v,
                    None => istr[2].parse::<i64>().unwrap()
                };
                if valx > 0 {
                    line = ((line as i64) + valy) as usize;
                    continue;
                }
            },
            _ => {},
        }
        line += 1;
    }

    ret.to_string()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day18_test1() {
        assert_eq!("4", Day18.solve_a("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"));
    }
    #[test]
    fn day18_test2() {
        assert_eq!("3", Day18.solve_b("snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"));
    }
}
