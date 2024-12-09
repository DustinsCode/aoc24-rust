use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules: HashMap<u32, Vec<u32>> =
        sections[0].split('\n').fold(HashMap::new(), |mut map, s| {
            let numbers: Vec<u32> = s.split('|').map(|n| n.parse::<u32>().unwrap()).collect();
            let key = numbers[0];
            let value = numbers[1..].to_vec();

            map.entry(key).or_default().extend(value);
            map
        });
    let pages: Vec<Vec<u32>> = sections[1]
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut total_middle_page = 0;

    for page in pages {
        if is_valid(&page, &rules) {
            total_middle_page += page[page.len() / 2]
        }
    }

    Some(total_middle_page)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let rules: HashMap<u32, Vec<u32>> =
        sections[0].split('\n').fold(HashMap::new(), |mut map, s| {
            let numbers: Vec<u32> = s.split('|').map(|n| n.parse::<u32>().unwrap()).collect();
            let key = numbers[0];
            let value = numbers[1..].to_vec();

            map.entry(key).or_default().extend(value);
            map
        });
    let pages: Vec<Vec<u32>> = sections[1]
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut total_middle_page = 0;

    for page in pages {
        if !is_valid(&page, &rules) {
            total_middle_page += sort_page(&page, &rules)[page.len() / 2]
        }
    }

    Some(total_middle_page)
}

pub fn is_valid(page: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, page_num) in page.iter().enumerate() {
        if rules.keys().any(|k| k == page_num)
            && rules[page_num].iter().any(|n| page[0..i].contains(n))
        {
            return false;
        }
    }

    true
}

pub fn sort_page(page: &[u32], rules: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
