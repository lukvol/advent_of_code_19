fn has_double_digit(number: i32) -> bool {
    let converted_input = number.to_string();
    let mut digits = converted_input.chars();
    let mut first_digit = digits.next().unwrap();
    for digit in digits {
        if first_digit == digit {
            return true
        }
        first_digit = digit;
    }
    false
}

fn is_non_decreasing(number: i32) -> bool {
    let converted_input = number.to_string();
    let mut digits = converted_input.chars();
    let mut first_digit = digits.next().unwrap();
    for digit in digits {
        if digit < first_digit {
            return false
        }
        first_digit = digit;
    }
    true
}

fn contains_small_group_of_matching_digits(number: i32) -> bool {
    let converted_input = number.to_string();
    let mut digits = converted_input.chars();
    let mut first_digit = digits.next().unwrap();

    let mut num_consecutive_matches = 0;

    for digit in digits {
        if first_digit == digit {
            num_consecutive_matches += 1;
        } else {
            if num_consecutive_matches == 1 {
                return true
            }
            num_consecutive_matches = 0;
        }
        first_digit = digit;
    }

    if num_consecutive_matches == 1 {
        true
    } else {
        false
    }
}

fn main() {
    let start = 402328;
    let end = 864247;

    let mut digits: Vec<i32> = Vec::new();
    for i in start..end + 1 {
        digits.push(i);
    }

    let double_digits: Vec<i32> = digits.into_iter().filter(|x| has_double_digit(*x)).collect();
    let double_digits_non_decreasing: Vec<i32> = double_digits.into_iter().filter(|x| is_non_decreasing(*x)).collect();

    println!("Solution for part 1: {}", double_digits_non_decreasing.len());

    let short_double_digits_non_decreasing: Vec<i32> = double_digits_non_decreasing.into_iter().filter(|x| contains_small_group_of_matching_digits(*x)).collect();
    
    println!("Solution for part 2: {}", short_double_digits_non_decreasing.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_number_with_double_digit() {
        let result = has_double_digit(124122145);
        assert!(result);
    }

    #[test]
    fn check_number_without_double_digit() {
        let result = has_double_digit(121212121);
        assert!(!result)
    }

    #[test]
    fn check_decreasing_number() {
        let result = is_non_decreasing(1111110);
        assert!(!result)
    }

    #[test]
    fn check_non_decreasing_number() {
        let result = is_non_decreasing(111112);
        assert!(result)
    }
    
    #[test]
    fn check_short_group_of_matching_digits_number() {
        assert!(contains_small_group_of_matching_digits(111122));
        assert!(contains_small_group_of_matching_digits(112233));
    }

    #[test]
    fn check_large_group_of_matching_digits_number() {
        assert!(!contains_small_group_of_matching_digits(123444));
    }
}