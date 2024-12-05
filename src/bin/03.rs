use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let digit_regex = Regex::new(r"\d+").unwrap();

    let matches: Vec<_> = mul_regex.find_iter(input).map(|x| x.as_str()).collect();
    let mut total = 0;

    for item in matches {
        let numbers: Vec<u32> = digit_regex
            .find_iter(item)
            .map(|d| d.as_str())
            .map(|d| d.parse::<u32>().unwrap())
            .collect();
        total += numbers[0] * numbers[1];
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();
    let digit_regex = Regex::new(r"\d+").unwrap();

    let matches: Vec<_> = mul_regex.find_iter(input).map(|x| x.as_str()).collect();
    let mut total = 0;
    let mut skip = false;
    
    for item in matches {
        if item == "don't()" {
            skip = true;
            continue;
        } else if item == "do()" {
            skip = false;
            continue;
        }
        
        if !skip {
            let numbers: Vec<u32> = digit_regex
                .find_iter(item)
                .map(|d| d.as_str())
                .map(|d| d.parse::<u32>().unwrap())
                .collect();
            total += numbers[0] * numbers[1];
        }
    }
    
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
