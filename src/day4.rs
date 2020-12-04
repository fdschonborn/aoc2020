#[derive(Debug, Default)]
pub struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|line| line.replace("\n", " "))
        .map(|s| {
            let mut pass = Passport::default();
            s.split(" ")
                .map(|field| {
                    let split = field.split(':').collect::<Vec<_>>();

                    (split[0], split[1])
                })
                .for_each(|(key, value)| {
                    let value = value.to_owned();

                    match key {
                        "byr" => pass.byr = Some(value),
                        "iyr" => pass.iyr = Some(value),
                        "eyr" => pass.eyr = Some(value),
                        "hcl" => pass.hcl = Some(value),
                        "ecl" => pass.ecl = Some(value),
                        "hgt" => pass.hgt = Some(value),
                        "pid" => pass.pid = Some(value),
                        "cid" => pass.cid = Some(value),
                        _ => unreachable!(),
                    }
                });

            pass
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|pass| {
            pass.byr.is_some()
                && pass.iyr.is_some()
                && pass.eyr.is_some()
                && pass.hcl.is_some()
                && pass.ecl.is_some()
                && pass.hgt.is_some()
                && pass.ecl.is_some()
                && pass.pid.is_some()
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|pass| {
            pass.byr.is_some()
                && pass.iyr.is_some()
                && pass.eyr.is_some()
                && pass.hcl.is_some()
                && pass.ecl.is_some()
                && pass.hgt.is_some()
                && pass.ecl.is_some()
                && pass.pid.is_some()
        })
        .filter(|pass| {
            let byr = dbg!(pass.byr.as_ref().unwrap().parse::<usize>().unwrap());
            if byr < 1920 || byr > 2002 {
                return false;
            }

            let iyr = dbg!(pass.iyr.as_ref().unwrap().parse::<usize>().unwrap());
            if iyr < 2010 || iyr > 2020 {
                return false;
            }

            let eyr = dbg!(pass.eyr.as_ref().unwrap().parse::<usize>().unwrap());
            if eyr < 2020 || eyr > 2030 {
                return false;
            }

            let hgt = dbg!(pass.hgt.as_ref().unwrap());
            let hgt_unit = dbg!(&hgt[hgt.len() - 2..]);
            let hgt_value = dbg!(hgt[..hgt.len() - 2].parse::<usize>().unwrap());
            match hgt_unit {
                "cm" => {
                    if hgt_value < 150 || hgt_value > 193 {
                        return false;
                    }
                }
                "in" => {
                    if hgt_value < 59 || hgt_value > 76 {
                        return false;
                    }
                }
                _ => return false,
            }

            let hcl = dbg!(pass.hcl.as_ref().unwrap());
            if hcl.chars().nth(0).unwrap() != '#'
                || !hcl[1..].chars().all(|c| c.is_ascii_hexdigit())
            {
                return false;
            }

            let ecl = dbg!(pass.ecl.as_ref().unwrap());
            if !matches!(
                ecl.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ) {
                return false;
            }

            let pid = dbg!(pass.pid.as_ref().unwrap());
            if pid.len() != 9 {
                return false;
            }

            true
        })
        .count()
}
