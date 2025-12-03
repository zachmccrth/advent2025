use std::fs::File;
use std::io::{Read, Result};

fn get_max_battery(battery_bank: &str) -> u32 {
    let mut max1 = 0;
    let mut max2 = 0;
    let byte_values = battery_bank.bytes();
    let length = battery_bank.len();
    for (i, byte) in byte_values.enumerate() {
        // if current value is greater than max1, and its not the last digit (max1 must be the
        // first digit in the pair), then consume it, else continue.
        if (byte > max1) & (i <= length - 2) {
            //Note, last digit in string is right before the
            //null terminator (second to last value)
            max2 = 0;
            max1 = byte;
            continue;
        };
        // if we haven't consumed the value yet, maybe we can consume it with max2
        if byte > max2 {
            max2 = byte
        };
    }
    String::from_utf8(vec![max1, max2])
        .expect("not a string")
        .parse()
        .expect("not a number")
}

fn insert_digit(digits: &mut Vec<u8>, digit: u8, remaining_digits: usize) {
    // First, strip all digits from end that are less than the new digit under consideration.
    // Note, we should only pop if we are confident that we can replace with enough digits
    let digits_to_pop = remaining_digits - (12 - digits.len());
    for _ in 0..digits_to_pop {
        let _ = digits.pop_if(|current_digit| *current_digit < digit);
    }

    // Then, if we have space, greedily add the new digit
    if digits.len() < 12 {
        digits.push(digit);
    }
}

fn get_max_battery_general(battery_bank: &str) -> u64 {
    let bytes = battery_bank.bytes();
    let battery_bank_length = bytes.len();

    let mut digits: Vec<u8> = Vec::new();

    for (i, byte) in bytes.enumerate() {
        insert_digit(&mut digits, byte, battery_bank_length - i);
    }

    assert!(digits.len() == 12, "actual length was {}", digits.len());

    str::from_utf8(digits.as_slice())
        .expect("Not a valid utf8 string")
        .parse()
        .expect("String is not a valid number")
}

fn main() -> Result<()> {
    let mut contents = String::new();
    {
        let _ = File::read_to_string(&mut File::open("input.txt")?, &mut contents);
    }

    let mut sum = 0;
    for battery_bank in contents.lines() {
        let battery_bank = battery_bank.trim();
        let max_battery = get_max_battery(battery_bank);
        sum += max_battery;
    }
    println!("Part 1 solution {}", sum);

    let mut sum = 0;
    for battery_bank in contents.lines() {
        let battery_bank = battery_bank.trim();
        let max_battery = get_max_battery_general(battery_bank);
        sum += max_battery;
    }
    println!("Part 2 solution {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let advent_examples = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let advent_answers = vec![987654321111, 811111111119, 434234234278, 888911112111];

        for (example, answer) in advent_examples.iter().zip(advent_answers) {
            let calced = get_max_battery_general(example);
            assert!(answer == calced, "answer {:?} given {:?}", answer, calced);
        }
    }
}
