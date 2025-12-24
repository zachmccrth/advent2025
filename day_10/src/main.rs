use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::{Display, write};
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::usize;

fn read_input() -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut contents);

    contents
}

fn get_delimiters_index(
    input_line: &str,
    (open_delimiter, close_delimiter): (char, char),
) -> (usize, usize) {
    let first_index = input_line.find(open_delimiter).expect(&format!(
        "Did not find delimiter {} in {}",
        open_delimiter, input_line
    ));

    let last_index = input_line.find(close_delimiter).expect(&format!(
        "Did not find delimiter {} in {}",
        close_delimiter, input_line
    ));

    (first_index, last_index)
}

fn get_lights(input_line: &str) -> Vec<bool> {
    let mut output = Vec::new();

    let (first_index, last_index) = get_delimiters_index(input_line, ('[', ']'));

    for element in input_line[(first_index + 1)..last_index].chars() {
        match element {
            '.' => output.push(false),
            '#' => output.push(true),
            _ => panic!("exceeded bounds of bank {}", input_line),
        }
    }

    output
}

fn get_buttons(input_line: &str) -> Vec<Vec<usize>> {
    let mut buttons = Vec::new();

    let mut input_iter = input_line.chars();

    loop {
        let char = input_iter.next();

        match char {
            Some('(') => {
                let mut num = String::new();
                let mut button: Vec<usize> = Vec::new();
                loop {
                    let char_option = input_iter.next();
                    match char_option {
                        Some(',') => {
                            button.push(num.parse().unwrap());
                            num = String::new();
                        }
                        Some(' ') => {}
                        Some(')') => {
                            button.push(num.parse().unwrap());
                            break;
                        }
                        Some(digit) => {
                            num.push(digit);
                        }
                        None => {
                            panic!("No matching ')' found in string {}", input_line)
                        }
                    }
                }
                buttons.push(button);
            }
            Some(_) => {}
            None => {
                break;
            }
        }
    }

    buttons
}

fn get_numeric(input_line: &str, delimiters: (char, char)) -> Vec<u32> {
    let mut output = Vec::new();

    let (first_index, last_index) = get_delimiters_index(input_line, delimiters);

    for element in input_line[(first_index + 1)..last_index].split(',') {
        let value = element
            .parse()
            .expect(&format!("Element {} was evidently not a value", element));
        output.push(value);
    }
    output
}

fn parse(input: &str) -> Vec<Problem> {
    let mut output = Vec::new();

    for line in input.lines() {
        let lights = get_lights(line);
        let buttons = get_buttons(line);
        let batteries = get_numeric(line, ('{', '}'));

        output.push(Problem {
            lights: lights,
            buttons: buttons,
            batteries: batteries,
        })
    }

    output
}

fn xor_state(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut new_state = state.clone();

    for index in button {
        new_state[*index] = !new_state[*index];
    }

    new_state
}

struct Problem {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    batteries: Vec<u32>,
}

// We could do some complicated stuff here, or we could just brute force
// Also, going to just allow the same button to be pressed twice in a row,
// even though it will be slightly slower (but faster to write!)
fn solve_problem_1(problem: &Problem) -> u64 {
    let Problem {
        lights,
        buttons,
        batteries: _,
    } = problem;

    let mut index_iterator = (0..buttons.len()).powerset();

    index_iterator.next();

    let mut presses = 0;
    for route in index_iterator {
        let mut state = vec![false; lights.len()];
        let route_len = route.len();
        for button_index in route {
            state = xor_state(&state, &buttons[button_index]);
            if state.eq(lights) {
                break;
            };
        }
        if state.eq(lights) {
            presses = route_len;
            break;
        };
    }

    presses as u64
}

fn fast_route(buttons: Vec<Vec<usize>>, answer: Vec<u32>) -> u32 {
    // if we assume that the minimum presses (the max num) is the solution, then we know that we
    // can only use a subset of the buttons -> the ones that include the max
    let max = answer.iter().max().unwrap();

    let max_indexes: Vec<usize> = (0..answer.len()).filter(|i| &answer[*i] == max).collect();

    let mut sorted_index = (0..answer.len()).sorted_by(|i, j| Ord::cmp(&answer[*i], &answer[*j]));

    let max_index = sorted_index.next().unwrap();

    let valid_buttons: Vec<Vec<usize>> = buttons
        .clone()
        .into_iter()
        .filter(|button| button.contains(&max_index))
        .collect();

    0
}

fn part_1(input: &str) -> u64 {
    let problems = parse(&input);

    let mut sum = 0;
    for problem in problems {
        sum += solve_problem_1(&problem);
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let problems = parse(&input);

    let mut sum = 0;

    sum
}
fn main() {
    let input = read_input();
    // let part_1_answer = part_1(&input);
    // println!("Part 1 Solution {}", part_1_answer);

    let part_2_answer = part_2(&input);
    println!("Part 2 Solution {}", part_2_answer);
}

#[derive(Clone)]
struct Matrix {
    values: Vec<i32>,
    size: (usize, usize),
}

impl Matrix {
    fn new(values_to_insert: Vec<Vec<usize>>) -> Matrix {
        let num_rows = values_to_insert.iter().flatten().max().unwrap() + 1;
        let num_cols = values_to_insert.len();

        let mut values = vec![0; num_rows * num_cols];

        for (col, value_rows) in values_to_insert.iter().enumerate() {
            for &row in value_rows {
                values[row * num_cols + col] = 1;
            }
        }

        Matrix {
            values,
            size: (num_rows, num_cols),
        }
    }

    fn get_row_range(self: &Self, row_index: usize) -> Range<usize> {
        (row_index * self.size.1)..((row_index + 1) * self.size.1)
    }

    fn get_row(self: &Self, row_index: usize) -> &[i32] {
        return &self.values[self.get_row_range(row_index)];
    }

    fn get_column(self: &Self, column_index: usize) -> Vec<i32> {
        return self
            .values
            .iter()
            .enumerate()
            .filter(|(i, _)| (i % self.size.1) == column_index)
            .map(|(_, &value)| value)
            .collect();
    }

    fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        let row_size = self.size.1;

        let (a, b) = if a < b { (a, b) } else { (b, a) };

        let (left, right) = self.values.split_at_mut(b * row_size);

        let row_a = &mut left[a * row_size..(a + 1) * row_size];
        let row_b = &mut right[0..row_size];

        row_a.swap_with_slice(row_b);
    }

    fn write_row(self: &mut Self, row_index: usize, row: &[i32]) {
        let range = self.get_row_range(row_index);
        self.values[range].copy_from_slice(row);
    }

    fn add_row_linear(
        self: &mut Self,
        coefficient: i32,
        row_operator_index: usize,
        row_recieve_index: usize,
    ) {
        let mut row_operator: Vec<i32> = self
            .get_row(row_operator_index)
            .iter()
            .map(|x| x * coefficient)
            .collect();

        let row_recieve = self.get_row(row_recieve_index);

        for i in 0..row_operator.len() {
            row_operator[i] = row_operator[i] + row_recieve[i];
        }

        self.write_row(row_recieve_index, row_operator.as_slice());
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        for row_index in 0..self.size.0 {
            for col_index in 0..self.size.1 {
                display.push_str(&self.values[(row_index * self.size.1) + col_index].to_string());
            }
            display.push('\n');
        }
        f.write_str(&display)
    }
}

#[derive(Debug)]
struct VectorEquation {
    const_values: Vec<i32>,
    free_variables: Vec<Vec<i32>>,
}
// impl VectorEquation {
//     // returns the minimum sum
//     fn get_minimum(self: &Self) -> u32 {
//         let mut values = Vec::new();
//         for free_variable in self.free_variables {
//             values.push(free_variable.iter().sum());
//         }
//
//         // numbers that are negative are good to add, 0s are free, and positive numbers have cost.
//         // So, we add the max we can of negative numbers, the least amount of positive numbers, and
//         // any amount of 0s
//
//         for index in (0..values.len())
//             .sorted_by(|i, j| Ord::cmp(&values[*i], &values[*j]))
//             .into_iter()
//         {
//             let value = values[index];
//             let free_variable = self.free_variables[index];
//         }
//
//         0
//     }
// }

fn gaussian_elimination(matrix: &Matrix, b: &mut Vec<i32>) -> VectorEquation {
    let mut row_index = 0;
    let mut col_index = 0;
    let min_matrix_size = min(matrix.size.0, matrix.size.1);
    let mut matrix = matrix.clone();
    while (row_index < matrix.size.0) & (col_index < matrix.size.1) {
        // find a row that matches the specifications well (has a 1, zero values for rest
        // (ideally))
        let mut l1 = i32::MAX;
        let mut row_operator_index = row_index;
        for row_index in (row_index..matrix.size.0) {
            let row = matrix.get_row(row_index);
            if row[col_index] != 0 {
                if row.iter().map(|x| x.abs()).sum::<i32>() < l1 {
                    row_operator_index = row_index;
                    l1 = row.iter().map(|x| x.abs()).sum::<i32>();
                }
            }
        }

        // normalize row (hopefully not necessary)
        let row_operator = matrix.get_row(row_operator_index);

        if row_operator[col_index] != 1 {
            if row_operator[col_index] == 0 {
                // just iterate the col index, we didn't find a valid row for this one
                col_index += 1;
                continue;
            } else if b[row_operator_index].abs() % row_operator[col_index].abs() != 0 {
                panic!(
                    "Agggh, the row is {:?} with row index {} and we are trying {} and we tried to normalize it but b is {:?} so the result is {}",
                    row_operator,
                    row_index,
                    col_index,
                    b,
                    b[row_operator_index].abs() % row_operator[col_index].abs()
                )
            } else {
                b[row_operator_index] = b[row_operator_index] / row_operator[col_index];

                let new_row_operator: Vec<i32> = row_operator
                    .iter()
                    .map(|x| {
                        assert!(x % row_operator[col_index] == 0);
                        x / row_operator[col_index]
                    })
                    .collect();
                matrix.write_row(row_operator_index, new_row_operator.as_slice());
            }
        }

        // swap to position
        //
        matrix.swap_rows(row_operator_index, row_index);
        b.swap(row_operator_index, row_index);
        row_operator_index = row_index;

        for row_recieve_index in 0..matrix.size.0 {
            if row_recieve_index == row_operator_index {
                continue;
            }
            let coefficient = -matrix.get_row(row_recieve_index)[col_index];
            matrix.add_row_linear(coefficient, row_operator_index, row_recieve_index);
            b[row_recieve_index] = b[row_recieve_index] + coefficient * b[row_operator_index];
        }

        row_index += 1;
        col_index += 1;
    }

    // identify free variables
    let mut pivot_columns = Vec::new();
    let mut free_columns = Vec::new();

    for row_index in 0..matrix.size.0 {
        let row = matrix.get_row(row_index);

        let col_option = row.iter().position(|&x| x == 1);
        if let Some(col_index) = col_option {
            pivot_columns.push(col_index);
        }
    }

    for col_index in 0..min(matrix.size.0, matrix.size.1) {
        if !pivot_columns.contains(&col_index) {
            let mut column = matrix.get_column(col_index);
            column[col_index] = 1;
            free_columns.push(column);
        }
    }

    VectorEquation {
        const_values: b.to_vec(),
        free_variables: free_columns,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_example_1() {
    //     let example_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    //     let example_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    //     let example_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    //
    //     let example_1_answer = part_1(example_1);
    //     let example_2_answer = part_1(example_2);
    //     let example_3_answer = part_1(example_3);
    //
    //     assert_eq!(example_1_answer, 2);
    //     assert_eq!(example_2_answer, 3);
    //     assert_eq!(example_3_answer, 2);
    // }
    //
    // #[test]
    // fn test_example_2() {
    //     let example_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    //     let example_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    //     let example_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    //
    //     let example_1_answer = part_2(example_1);
    //     let example_2_answer = part_2(example_2);
    //     let example_3_answer = part_2(example_3);
    //
    //     assert_eq!(example_1_answer, 10);
    //     assert_eq!(example_2_answer, 12);
    //     assert_eq!(example_3_answer, 11);
    // }
    //
    // #[test]
    // fn test_delimiters() {
    //     let test_example = "(2,3) (4,3,2)";
    //     let answer = vec![vec![2, 3], vec![4, 3, 2]];
    //
    //     assert_eq!(get_buttons(test_example), answer);
    //
    //     let test_example = "[..##..#]";
    //     let answer = vec![false, false, true, true, false, false, true];
    //
    //     assert_eq!(get_lights(test_example), answer);
    // }

    #[test]
    fn test_gaussian() {
        let example_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let example_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let example_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let mut problems = parse(&example_1);
        problems.append(&mut parse(&example_2));
        problems.append(&mut parse(&example_3));

        for problem in problems {
            let matrix = Matrix::new(problem.buttons);
            let mut b = problem.batteries.iter().map(|x| *x as i32).collect();

            println!("{}", matrix);
            println!("{:?}", b);
            let solution = gaussian_elimination(&matrix, &mut b);
            println!("{:?}", solution);
        }
    }
    #[test]
    fn test_input() {
        let input = read_input();

        let problems = parse(&input);

        for problem in problems {
            let matrix = Matrix::new(problem.buttons);
            let mut b = problem.batteries.iter().map(|x| *x as i32).collect();

            println!("{}", matrix);
            println!("{:?}", b);
            let solution = gaussian_elimination(&matrix, &mut b);
            println!("{:?}", solution);
        }
    }
}
