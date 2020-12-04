use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::RangeBounds, str::FromStr};

fn numeric_and_in_range<T>(s: Option<&str>, range: impl RangeBounds<T>) -> bool
where
    T: FromStr + PartialOrd,
{
    let v = match s.and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return false,
    };

    range.contains(&v)
}

#[derive(Debug, Default)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}
impl<'a> Passport<'a> {
    #[allow(clippy::unnecessary_wraps)]
    fn get_mut_value_slot(&mut self, key: &str) -> Option<&mut Option<&'a str>> {
        Some(match key {
            "byr" => &mut self.byr,
            "iyr" => &mut self.iyr,
            "eyr" => &mut self.eyr,
            "hgt" => &mut self.hgt,
            "hcl" => &mut self.hcl,
            "ecl" => &mut self.ecl,
            "pid" => &mut self.pid,
            "cid" => &mut self.cid,
            _ => return None,
        })
    }

    fn set_key(&mut self, key: &str, value: &'a str) -> bool {
        if let Some(slot) = self.get_mut_value_slot(key) {
            slot.replace(value);
            true
        } else {
            false
        }
    }

    fn set_from_pair(&mut self, pair: &'a str) -> bool {
        let mut parts = pair.split(':');
        let key = match parts.next() {
            Some(v) => v,
            None => return false,
        };
        let value = match parts.next() {
            Some(v) => v,
            None => return false,
        };
        if parts.next().is_some() {
            false
        } else {
            self.set_key(key, value)
        }
    }

    fn has_required_fields(&self) -> bool {
        let Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid: _,
        } = self;
        byr.is_some()
            && iyr.is_some()
            && eyr.is_some()
            && hgt.is_some()
            && hcl.is_some()
            && ecl.is_some()
            && pid.is_some()
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
        }

        let hgt_valid = if let Some(hgt) = self.hgt {
            if let Some(v) = hgt.strip_suffix("cm") {
                numeric_and_in_range(Some(v), 150..=193)
            } else if let Some(v) = hgt.strip_suffix("in") {
                numeric_and_in_range(Some(v), 59..=76)
            } else {
                false
            }
        } else {
            false
        };
        let hcl_valid = self.hcl.map_or(false, |hcl| HCL_RE.is_match(hcl));

        hgt_valid
            && hcl_valid
            && numeric_and_in_range(self.byr, 1920..=2002)
            && numeric_and_in_range(self.iyr, 2010..=2020)
            && numeric_and_in_range(self.eyr, 2020..=2030)
            && matches!(
                self.ecl.unwrap_or_default(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
            && self.pid.map_or(false, |pid| pid.len() == 9)
    }
}

fn parse_input(inp: &str) -> Option<Vec<Passport>> {
    let inp = inp.trim();
    let mut passports = Vec::default();
    let mut passport = Passport::default();
    for line in inp.lines().map(|s| s.trim()) {
        if line.is_empty() {
            passports.push(std::mem::take(&mut passport));
            continue;
        }

        for pair in line.split_whitespace() {
            if !passport.set_from_pair(pair) {
                println!("{} | {}", pair, line);
                return None;
            }
        }
    }

    if !inp.is_empty() {
        passports.push(passport);
    }

    Some(passports)
}

fn first_part<'a>(passports: impl IntoIterator<Item = &'a Passport<'a>>) -> usize {
    passports
        .into_iter()
        .filter(|p| p.has_required_fields())
        .count()
}

fn second_part<'a>(passports: impl IntoIterator<Item = &'a Passport<'a>>) -> usize {
    passports.into_iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in   
    "#;

    #[test]
    fn first() {
        let sol = first_part(&parse_input(EXAMPLE_INPUT).expect("failed to parse input"));
        assert_eq!(sol, 2);
    }

    const VALID_PASSPORTS: &str = r#"
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        
        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    "#;
    const INVALID_PASSPORTS: &str = r#"
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007
    "#;

    #[test]
    fn second_valid() {
        let sol = second_part(&parse_input(VALID_PASSPORTS).expect("failed to parse input"));
        assert_eq!(sol, 4);
    }

    #[test]
    fn second_invalid() {
        let sol = second_part(&parse_input(INVALID_PASSPORTS).expect("failed to parse input"));
        assert_eq!(sol, 0);
    }
}
