use std::{fs::File, io::Read, ops::RangeInclusive};

fn read_input() -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut contents);

    contents
}

fn parse_ranges(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut lines = input.lines().into_iter();

    let mut output_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        match line.split_once('-') {
            Some((start, end)) => output_ranges.push(RangeInclusive::new(
                start.parse().unwrap(),
                end.parse().unwrap(),
            )),
            None => {
                break;
            }
        }
    }

    let mut output_values: Vec<u64> = Vec::new();
    for line in lines {
        let value: u64 = line.parse().unwrap();
        output_values.push(value);
    }

    (output_ranges, output_values)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum NumericType {
    // the order here means that a RangeStart is less than a Value wich is less than a RangeEnd
    // Therefore, this is inclusive ordering
    RangeStart,
    Value,
    RangeEnd,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ValueOrRangeStruct {
    // Here the order causes derive to make value more important than the type of value
    // The numeric type will only be important when the values are equal (and will ensure that
    // the value lies between its ranges)
    value: u64,
    kind: NumericType,
}

fn construct_vector_to_sort(
    ranges: Vec<RangeInclusive<u64>>,
    values: Vec<u64>,
) -> Vec<ValueOrRangeStruct> {
    let mut output: Vec<ValueOrRangeStruct> = Vec::new();
    for range in ranges {
        // Push the start
        output.push(ValueOrRangeStruct {
            value: *range.start(),
            kind: NumericType::RangeStart,
        });
        // Push the end
        output.push(ValueOrRangeStruct {
            value: *range.end(),
            kind: NumericType::RangeEnd,
        });
    }

    for value in values {
        output.push(ValueOrRangeStruct {
            value: value,
            kind: NumericType::Value,
        });
    }

    output
}

fn count_fresh_ingredients(sorted_vec: &Vec<ValueOrRangeStruct>) -> u32 {
    let mut sum = 0;

    let mut current_active_ranges = 0;
    for element in sorted_vec {
        match element.kind {
            NumericType::RangeStart => {
                current_active_ranges += 1;
            }
            NumericType::Value => {
                if current_active_ranges > 0 {
                    sum += 1
                };
            }
            NumericType::RangeEnd => {
                current_active_ranges -= 1;
            }
        }
    }

    sum
}

fn count_possible_fresh_ingredients(sorted_vec: &Vec<ValueOrRangeStruct>) -> u64 {
    let mut output = 0;

    let mut number_of_active_ranges = 0;
    let mut start_of_active_range: u64 = 0;
    for element in sorted_vec {
        match element.kind {
            NumericType::RangeStart => {
                if number_of_active_ranges == 0 {
                    start_of_active_range = element.value;
                }
                number_of_active_ranges += 1;
            }
            NumericType::Value => {}
            NumericType::RangeEnd => {
                number_of_active_ranges -= 1;
                if number_of_active_ranges == 0 {
                    output += element.value - start_of_active_range + 1; // +1 for inclusive range
                }
            }
        }
    }

    output
}

fn part_1(input: &str) -> u32 {
    let (ranges, values) = parse_ranges(&input);

    let mut to_sort = construct_vector_to_sort(ranges, values);

    to_sort.sort();

    count_fresh_ingredients(&to_sort)
}

fn part_2(input: &str) -> u64 {
    let (ranges, values) = parse_ranges(&input);

    let mut to_sort = construct_vector_to_sort(ranges, values);

    to_sort.sort();

    count_possible_fresh_ingredients(&to_sort)
}
fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Part 1 Answer: {}", part_1_answer);

    let part_2_answer = part_2(&input);

    println!("Part 2 Answer: {}", part_2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 3);

        let part_2_answer = part_2(&input);

        assert_eq!(part_2_answer, 14);
    }
}
