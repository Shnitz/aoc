use aoc::*;
use days::ChristmasDay;
// use na::{Matrix, Matrix2, Matrix3, Matrix4, MatrixVec, DMatrix};

pub struct Day21;

impl ChristmasDay for Day21 {
    fn solve(&self, data: &str, _prob: ProblemPart) -> String {
        let mut img: Vec<u8> = vec![0,1,0,0,0,1,1,1,1];

        let mut rules3: Vec<(Vec<u8>, Vec<u8>)> = vec![];
        let mut rules2: Vec<(Vec<u8>, Vec<u8>)> = vec![];

        for line in data.lines() {
            let kv = line.split(" => ").collect::<Vec<&str>>();
            let rs = kv[0].split("/").flat_map(|r: &str| r.chars().map(|c| match c {
                '.' => 0, '#' => 1, _ => panic!("Invalid char!")
            }).collect::<Vec<u8>>()).collect::<Vec<u8>>();
            let os = kv[1].split("/").flat_map(|r: &str| r.chars().map(|c| match c {
                '.' => 0, '#' => 1, _ => panic!("Invalid char!")
            }).collect::<Vec<u8>>()).collect::<Vec<u8>>();
            match rs.len() {
                x if x % 2 == 0 => { rules2.push((rs, os)); },
                x if x % 3 == 0 => { rules3.push((rs, os)); },
                _ => panic!("Bad size!")
            }
        }
        for stp in 0..5 {
            let size = (img.len() as f64).sqrt() as usize;
            println!("Step {}", stp);
            for i in 0..size {
                println!("{:?}", &img[i*size..i*size+size]);
            }

            match img.len() {
                l if l % 3 == 0 => {
                    let w = img.len() / 9;
                    let mut imgn = vec![];
                    for _ in 0..w*16 { imgn.push(0); }

                    for y in 0..size/3 {
                        for x in 0..size/3 {
                            let p = x*3+y*size*3;
                            let sq = vec![img[p],img[p+1],img[p+2],
                                        img[p+size],img[p+size+1],img[p+size+2],
                                        img[p+2*size],img[p+2*size+1],img[p+2*size+2]];
                            let mut is_match = false;
                            'out: for rule in &rules3 {
                                let mut test = sq.clone();
                                for _ in 0..4 {
                                    if test == rule.0 || flip(&test) == rule.0 {
                                        let n = &rule.1;
                                        let wn = (imgn.len() as f64).sqrt() as usize;
                                        let pn = x*4+y*wn*4;
                                        for i in 0..4 {
                                            imgn[pn + i] = n[i];
                                            imgn[pn + wn + i] = n[i + 4];
                                            imgn[pn + 2*wn + i] = n[i + 8];
                                            imgn[pn + 3*wn + i] = n[i + 12];
                                        }
                                        //
                                        // let tst = (test.len() as f64).sqrt() as usize;
                                        // for i in 0..tst {
                                        //     println!("{:?}", &test[i*tst..i*tst+tst]);
                                        // }
                                        // let tst1 = (rule.1.len() as f64).sqrt() as usize;
                                        // println!("Matches");
                                        // for i in 0..tst1 {
                                        //     println!("{:?}", &rule.1[i*tst1..i*tst1+tst1]);
                                        // }

                                        is_match = true;
                                        break 'out;
                                    }
                                    test = rotate_r(&test);
                                }
                            }
                            if !is_match {
                                panic!("No match found");
                            }
                        }
                    }
                    img = imgn.clone();
                },
                l if l % 2 == 0 => {
                    let w = img.len() / 4;
                    let mut imgn = vec![];
                    for _ in 0..w*9 { imgn.push(0); }
                    for y in 0..size/2 {
                        for x in 0..size/2 {
                            let p = x*2+y*size*2;
                            let sq = vec![img[p],img[p+1],img[p+size],img[p+size+1]];
                            let mut is_match = false;
                            'out2: for rule in &rules2 {
                                let mut test = sq.clone();
                                for _ in 0..4 {
                                    if test == rule.0 || flip(&test) == rule.0 {
                                        let n = &rule.1;
                                        let wn = (imgn.len() as f64).sqrt() as usize;
                                        let pn = x*3+y*wn*3;
                                        for i in 0..3 {
                                            imgn[pn + i] = n[i];
                                            imgn[pn + wn + i] = n[i + 3];
                                            imgn[pn + 2*wn + i] = n[i + 6];
                                        }

                                        // let tst = (n.len() as f64).sqrt() as usize;
                                        // println!("NN");
                                        // for i in 0..tst {
                                        //     println!("{:?}", &n[i*tst..i*tst+tst]);
                                        // }

                                        is_match = true;
                                        break 'out2;
                                    }
                                    test = rotate_r(&test);
                                }
                            }
                            if !is_match {
                                panic!("No match found");
                            }
                        }
                    }
                    img = imgn.clone();
                },
                _ => {},
            }
        }
        let size = (img.len() as f64).sqrt() as usize;
        println!("Step Final");
        for i in 0..size {
            println!("{:?}", &img[i*size..i*size+size]);
        }
        img.iter().map(|&n| n as usize).sum::<usize>().to_string()
    }
}

fn flip(i: &Vec<u8>) -> Vec<u8> {
    match i.len() {
        9 => {
            vec![i[2],i[1],i[0],i[5],i[4],i[3],i[8],i[7],i[6]]
        },
        4 => {
            vec![i[1],i[0],i[3],i[2]]
        },
        _ => panic!("Bad Size"),
    }
}
fn rotate_r(i: &Vec<u8>) -> Vec<u8> {
    match i.len() {
        9 => {
            vec![i[6],i[3],i[0],i[7],i[4],i[1],i[8],i[5],i[2]]
        },
        4 => {
            vec![i[2],i[0],i[3],i[1]]
        },
        _ => panic!("Bad Size"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day21_test1() {
        assert_eq!("12", Day21.solve_a(
            "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"));
        // assert_eq!("", Day21.solve_b(""));
    }
}
