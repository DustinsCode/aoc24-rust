advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let pre_list = input.split('\n').collect::<Vec<&str>>();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for item in pre_list {
        if !item.is_empty() {
            let temp_arr = item.split("   ").collect::<Vec<&str>>();
            left.push(temp_arr[0].parse::<i32>().unwrap());
            right.push(temp_arr[1].parse::<i32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    for i in 0..left.len() {
        sum += (left[i] - right[i]).unsigned_abs();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pre_list = input.split('\n').collect::<Vec<&str>>();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for item in pre_list {
        if !item.is_empty() {
            let temp_arr = item.split("   ").collect::<Vec<&str>>();
            left.push(temp_arr[0].parse::<u32>().unwrap());
            right.push(temp_arr[1].parse::<u32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    for i in left {
        if right.contains(&i) {
            sum += i * right.iter().filter(|&n| *n == i).count() as u32;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
