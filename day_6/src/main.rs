use std::{fs::File, io::Read, u64};

fn read_input() -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut contents);

    contents
}

fn extract_values_operations(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let num_lines = input.lines().next().unwrap().split_whitespace().count();

    let mut values = vec![Vec::<u64>::new(); num_lines];
    let mut operations = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let numbers = line.split_whitespace();
        for (i, number) in numbers.enumerate() {
            let result = number.parse::<u64>();
            match result {
                Ok(value) => values[i].push(value),
                Err(_) => operations.push(number),
            }
        }
    }

    (values, operations)
}

fn generate_map(operation: &str) -> fn(u64, u64) -> u64 {
    match operation {
        "+" => std::ops::Add::add,
        "*" => std::ops::Mul::mul,
        &_ => panic!("Not a valid operation: {}", operation),
    }
}

fn part_1(input: &str) -> u64 {
    let (values, operations) = extract_values_operations(&input);

    let mut sum = 0;
    for (i, value_list) in values.iter().enumerate() {
        let op = generate_map(operations[i]);

        sum += value_list.iter().copied().reduce(op).unwrap_or(0);
    }
    sum
}

// Part 2 is going to have to be very different...

// lets just map to columns for simplicity this time

fn map_columns(input: &str) -> Vec<String> {
    let line_length = input.lines().next().unwrap().chars().count();

    let mut columns: Vec<String> = vec![String::new(); line_length];

    for line in input.lines() {
        for (i, character) in line.chars().enumerate() {
            columns[i].push(character);
        }
    }

    columns
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug)]
struct Block {
    values: Vec<u64>,
    operation: Operation,
}

impl Block {
    fn add_value(self: &mut Self, string: &mut String) {
        let last_char = string.pop().unwrap();
        if last_char == '*' {
            self.operation = Operation::Multiplication;
        } else if last_char == '+' {
            self.operation = Operation::Addition;
        }

        // filter out whitespaces and parse what is hopefully a number
        self.values.push(
            string
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap(),
        );
    }

    fn compute_result(self: &Self) -> u64 {
        match self.operation {
            Operation::Addition => self.values.iter().sum(),
            Operation::Multiplication => self.values.iter().product(),
        }
    }
}

fn part_2(input: &str) -> u64 {
    let columns = map_columns(&input);

    let mut blocks = Vec::new();

    let mut block = Block {
        values: Vec::new(),
        operation: Operation::Addition,
    };

    for mut column in columns {
        if column.trim() == "" {
            blocks.push(block);
            block = Block {
                values: Vec::new(),
                operation: Operation::Addition,
            };
        } else {
            block.add_value(&mut column);
        }
    }
    // last one doesn't have a delimiter
    blocks.push(block);

    let mut sum = 0;
    for block in blocks {
        sum += block.compute_result();
    }

    sum
}

fn main() {
    let input = read_input();

    let part_1_solution = part_1(&input);
    println!("Part 1 Solution {}", part_1_solution);

    let part_2_solution = part_2(&input);
    println!("Part 2 Solution {}", part_2_solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input_1 = String::from("123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +  ");
        let part_1_calc = part_1(&input_1);
        assert_eq!(part_1_calc, 4277556);
    }
    #[test]
    fn test_example_2() {
        let input_2 =
            String::from("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        let part_2_calc = part_2(&input_2);
        assert_eq!(part_2_calc, 3263827);
    }
}
