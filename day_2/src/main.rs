use std::fs::File;
use std::io::{self, Read};

fn is_invalid(number: &str) -> bool {
    let num_digits = number.chars().count();
    if num_digits % 2 == 1 {
        return false;
    }

    return number[..num_digits / 2] == number[num_digits / 2..];
}

fn get_divisors(number: i32) -> Vec<i32> {
    let mut divisors = Vec::new();
    for i in 1..=(number / 2) {
        if number % i == 0 {
            divisors.push(i);
        }
    }
    return divisors;
}

fn check_split(number_str: &str, num_split: usize) -> bool {
    for i in 0..((number_str.chars().count()) / num_split) - 1 {
        if number_str[i * num_split..(i + 1) * num_split]
            != number_str[(i + 1) * num_split..(i + 2) * num_split]
        {
            return false;
        }
    }
    true
}

fn is_invalid_2(number: &str) -> bool {
    let num_digits = number.chars().count();
    let divisors = get_divisors(num_digits as i32);

    for divisor in divisors {
        if check_split(number, divisor as usize) {
            return true;
        };
    }

    false
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents = contents.trim().to_string();

    let ranges = contents.split(",");

    let mut sum = 0;

    for range in ranges {
        let (start, end) = range.split_once("-").expect("Need delimiter");
        let start: i64 = start
            .parse()
            .expect(&format!("Invalid number: {:?}", start));
        let end: i64 = end.parse().expect(&format!("Invalid number: {:?}", end));

        for number in start..=end {
            if is_invalid(&number.to_string()) {
                sum += number;
            }
        }
    }

    println!("Part 1 Solution {}", sum);

    let ranges = contents.split(",");

    let mut sum = 0;

    for range in ranges {
        let (start, end) = range.split_once("-").expect("Need delimiter");
        let start: i64 = start
            .parse()
            .expect(&format!("Invalid number: {:?}", start));
        let end: i64 = end.parse().expect(&format!("Invalid number: {:?}", end));

        for number in start..=end {
            if is_invalid_2(&number.to_string()) {
                println!("Invalid Number: {}", number);
                sum += number;
            }
        }
    }

    println!("Part 2 Solution {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_divisors(10);
        dbg!(result);
    }
}
