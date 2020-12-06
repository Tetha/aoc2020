use std::collections::HashMap;

use crate::AdventError;

type Passport = HashMap<String, String>;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    println!("There are {} valid passports", count_valid_passports(input)?);
    Ok(())
}

fn count_valid_passports(input: &str) -> Result<usize, AdventError> {
    let passports = parse_passports(input)?;
    //println!("{:#?}", passports);
    Ok(passports.iter().filter(|p| is_valid(p)).count())
}
fn is_valid(passport: &Passport) -> bool {
    println!("Checking: {:#?}", passport);

    println!("fields");
    let has_all_fields = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ].iter().all(|field| passport.contains_key(*field));

    if !has_all_fields {
        false
    } else {
        println!("\tBirth year");
        let birth_year = passport["byr"].parse::<i32>();
        if let Ok (year) = birth_year {
            if year < 1920 || 2002 < year {
                return false
            }
        } else {
            return false;
        }

        println!("\tIssue year");
        let issue_year = passport["iyr"].parse::<i32>();
        if let Ok (year) = issue_year {
            if year < 2010 || 2020 < year {
                return false
            }
        } else {
            return false;
        }

        println!("\tExpiry year");
        let expiration_year = passport["eyr"].parse::<i32>();
        if let Ok (year) = expiration_year {
            if year < 2020 || 2030 < year {
                return false
            }
        } else {
            return false;
        }

        println!("\tHeight");
        let height = &passport["hgt"];
        if height.ends_with("in") {
            let height = height.trim_end_matches("in").parse::<i32>();
            if let Ok(height_in)= height {
                if height_in < 59 || 76 < height_in {
                    return false
                }
            } else {
                return false;
            }
        } else if height.ends_with("cm") {
            let height = height.trim_end_matches("cm").parse::<i32>();
            if let Ok(height_cm )= height {
                if height_cm < 150 || 193 < height_cm {
                    return false
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        println!("\tHair Color - {}, {}, {}",
            passport["hcl"].len() != 7,
            passport["hcl"].chars().nth(0) != Some('#'), 
            !passport["hcl"].chars().skip(1).all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f')));

        if passport["hcl"].len() != 7 
            || passport["hcl"].chars().nth(0) != Some('#')
            || !passport["hcl"].chars().skip(1).all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f')) {
                return false
            }

        println!("\t Eye Color");
        if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"].as_str()) {
            return false;
        }

        println!("\t Password ID");
        if passport["pid"].len() != 9 || !passport["pid"].chars().all(|c| '0' <= c && c <= '9') {
            return false;
        }

        println!("\t Valid");
        true
    }
}

fn parse_passports(input: &str) -> Result<Vec<Passport>, AdventError> {
    let mut result: Vec<Passport> = Vec::new();
    let mut current_passport = Passport::new();
    for line in input.lines() {
        if line == "" {
            result.push(current_passport);
            current_passport = Passport::new();
        } else {
            for part in line.trim().split(" ") {
                let chunks = part.split(":").collect::<Vec<&str>>();
                if chunks.len() != 2 {
                    return Err(AdventError{cause: format!("Line <{}> has invalid chunk <{}>", line, part)});
                }
                let key = chunks[0].to_string();
                let value = chunks[1].to_string();
                current_passport.insert(key, value);
            }
        }
    }
    if current_passport.len() != 0 {
        result.push(current_passport);
    }
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

        assert_eq!(2, count_valid_passports(input).unwrap());
    }

    #[test]
    fn test_last_passport() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

";

        assert_eq!(1, count_valid_passports(input).unwrap());
    }
}