use aoc::*;
use days::ChristmasDay;

pub struct Day13;

impl ChristmasDay for Day13 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let firewall = data.lines()
            .map(str::split_whitespace)
            .map(|mut i| (i.next().unwrap().trim_matches(':').parse::<u32>().unwrap(), i.next().unwrap().parse::<u32>().unwrap()))
            .collect::<Vec<(u32, u32)>>();

        match prob {
            ProblemPart::A => firewall.iter().map(self::firewall_cost).sum::<u32>().to_string(),
            ProblemPart::B => {
                let mut delay = 0;
                while firewall.iter().any(|p| firewall_cost(&(p.0 + delay, p.1)) != 0) {
                    delay += 1;
                }
                delay.to_string()
            }
        }

    }
}

fn firewall_cost(&(time, size): &(u32, u32)) -> u32 {
    assert!(size > 1);
    if time % (2 * (size - 1)) == 0 {
        size * time
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day13_test1() {
        let data = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!("23", Day13.solve_a(data));
        assert_eq!("10", Day13.solve_b(data));
    }
}
