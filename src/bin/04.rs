use std::string::String;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let xmas_str = "XMAS";
    let mut xmas_count: u32 = 0;

    let rows = input.split('\n').collect::<Vec<&str>>();

    let matrix: Vec<Vec<char>> = rows
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'X' {
                // check ->
                if x <= row.len() - 4 {
                    let word = matrix[y][x..x + 4].iter().collect::<String>();
                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check <-
                if x > 2 {
                    let word = matrix[y][x - 3..=x].iter().rev().collect::<String>();
                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ↓
                if y < matrix.len() - 4 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y + i][x].to_string().as_str());
                    }
                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ꜛ
                if y > 2 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y - i][x].to_string().as_str());
                    }
                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ↘
                if y < matrix.len() - 4 && x <= row.len() - 4 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y + i][x + i].to_string().as_str());
                    }

                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ↖
                if y > 2 && x > 2 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y - i][x - i].to_string().as_str());
                    }

                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ↗
                if y > 2 && x <= row.len() - 4 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y - i][x + i].to_string().as_str());
                    }

                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }

                // check ↙
                if y < matrix.len() - 4 && x > 2 {
                    let mut word = String::new();

                    for i in 0..4 {
                        word.push_str(matrix[y + i][x - i].to_string().as_str());
                    }

                    if word == xmas_str {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    Some(xmas_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
