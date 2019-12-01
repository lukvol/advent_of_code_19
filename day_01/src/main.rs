use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("./input.txt")?;
    let reader = BufReader::new(f);

    let mut sum_part_one: i64 = 0;
    let mut sum_part_two: i64 = 0;

    for line in reader.lines() {
        if let Ok(input) = line {
            if let Ok(mass) = input.parse::<i64>() {
                sum_part_one += calculate_fuel_for_part_one(mass);
                sum_part_two += calculate_fuel_for_part_two(mass);
            }
        }
    }

    println!("Solution for part one: {}", sum_part_one);
    println!("Solution for part two: {}", sum_part_two);

    Ok(())
}

fn calculate_fuel_for_part_one(mass: i64) -> i64 {
    (mass / 3 - 2).max(0)
}

fn calculate_fuel_for_part_two(mass: i64) -> i64 {
    if mass / 3 == 0 {
        return 0
    }

    let fuel = (mass / 3 - 2).max(0);
    fuel + calculate_fuel_for_part_two(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_for_second_part() {
        let result = calculate_fuel_for_part_two(100756);
        assert_eq!(result, 50346);
    }
}