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
