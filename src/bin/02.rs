advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;

    let rows = input.split('\n').collect::<Vec<&str>>();

    for row in rows {
        let row_items = row.split(' ').collect::<Vec<&str>>();

        if check_safety(row_items) && !row.is_empty() {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;

    let rows = input.split('\n').collect::<Vec<&str>>();

    for row in rows {
        let row_items = row.split(' ').collect::<Vec<&str>>();

        if (check_safety(row_items.clone()) && !row.is_empty()) {
            safe += 1
        } else if !row.is_empty() {
            for i in 0..row_items.len() {
                let copy = [&row_items[0..i], &row_items[i + 1..]].concat();
                if check_safety(copy) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    Some(safe)
}

pub fn check_safety(row: Vec<&str>) -> bool {
    let mut increasing = false;
    let mut is_safe = true;

    if row.len() == 1 {
        return true;
    }

    for i in 0..row.len() - 1 {
        let current_item = row[i].parse::<u32>().unwrap();
        let next_item = row[i + 1].parse::<u32>().unwrap();

        if i == 0 && current_item < next_item {
            increasing = true;
        }

        if !safe_step(current_item, next_item, increasing) {
            is_safe = false;
            break;
        }
    }

    is_safe
}

pub fn safe_step(first: u32, second: u32, increasing: bool) -> bool {
    if first == second {
        return false;
    }

    if (increasing && first > second) || (!increasing && first < second) {
        return false;
    }

    let diff = first.abs_diff(second);

    if !(1..=3).contains(&diff) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
