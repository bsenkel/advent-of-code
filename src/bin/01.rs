advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let commands: Vec<(&str, i32)> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let direction = &line[0..1];
            let steps: i32 = line[1..].parse().ok()?;
            Some((direction, steps))
        })
        .collect();

    println!("Number of input lines: {}", commands.len());

    let mut position: i32 = 50; // start
    let mut count_zero: u64 = 0;

    for (direction, steps) in commands {
        match direction {
            "R" => position += steps,
            "L" => position -= steps,
            _ => {}
        }

        while position > 99 {
            position -= 100;
        }
        while position < 0 {
            position += 100;
        }

        if position == 0 {
            count_zero += 1;
        }
    }

    Some(count_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands: Vec<(&str, i32)> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let direction = &line[0..1];
            let steps: i32 = line[1..].parse().ok()?;
            Some((direction, steps))
        })
        .collect();

    let mut position: i32 = 50; // start
    let mut count_zero: u64 = 0;

    for (direction, steps) in commands {
        for _ in 0..steps {
            match direction {
                "R" => position += 1,
                "L" => position -= 1,
                _ => {}
            }

            while position > 99 {
                position = 0;
            }
            while position < 0 {
                position = 99;
            }

            if position == 0 {
                count_zero += 1;
            }
        }
    }

    Some(count_zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
