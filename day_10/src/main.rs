use itertools::Itertools;
use std::cmp::{min, max};
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
    values: Vec<u32>,
    size: (usize, usize),
}

impl Matrix {
    fn new(values_to_insert: Vec<Vec<usize>>) -> Matrix {
        let num_cols = values_to_insert.iter().flatten().max().unwrap() + 1;
        let num_rows = values_to_insert.len();

        let mut values = vec![0; num_rows * num_cols];

        for (i, value_column) in values_to_insert.iter().enumerate() {
            for index in value_column {
                values[i * num_cols + index] = 1;
            }
        }

        Matrix {
            values: values,
            size: (num_rows, num_cols),
        }
    }

    fn get_row_range(self: &Self, row_index: usize) -> Range<usize> {
        (row_index * self.size.1)..((row_index + 1) * self.size.1)
    }

    fn get_row(self: &Self, row_index: usize) -> &[u32] {
        return &self.values[self.get_row_range(row_index)];
    }

    fn get_column(self: &Self, column_index: usize) -> Vec<u32> {
        return self
            .values
            .iter()
            .enumerate()
            .filter(|(i, _)| (i % self.size.1) == column_index)
            .map(|(_, &value)| value)
            .collect();
    }

    fn swap_rows(self: &mut Self, row_a_index: usize, row_b_index: usize) {

        if row_a_index == row_b_index {return;}

        let row_1_index = min(row_a_index, row_b_index);
        let row_2_index = max(row_a_index, row_b_index);

        let row_2 = self.get_row(row_2_index).to_vec();

        let (values_up_to_row_2, row_2_and_on)  = self.values.split_at_mut(row_2_index*self.size.1);


    }

    fn write_row(self: &mut Self, row_index: usize, row: &[u32]) {
        let range = self.get_row_range(row_index);
        self.values[range].copy_from_slice(row);
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

fn gaussian_elimination(matrix: &Matrix, b: Vec<u32>) -> () {
    let mut index = 0;
    let min_matrix_size = min(matrix.size.0, matrix.size.1);
    while index < min(matrix.size.0, matrix.size.1) {
        // find a row that matches the specifications well (has a 1, nonzero values for rest
        // (ideally))
        let mut selected_row = 
        for row_index in (index..matrix.size.0) {
            let row = matrix.get_row(row_index);
            if row
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1() {
        let example_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let example_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let example_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let example_1_answer = part_1(example_1);
        let example_2_answer = part_1(example_2);
        let example_3_answer = part_1(example_3);

        assert_eq!(example_1_answer, 2);
        assert_eq!(example_2_answer, 3);
        assert_eq!(example_3_answer, 2);
    }

    #[test]
    fn test_example_2() {
        let example_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let example_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let example_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let example_1_answer = part_2(example_1);
        let example_2_answer = part_2(example_2);
        let example_3_answer = part_2(example_3);

        assert_eq!(example_1_answer, 10);
        assert_eq!(example_2_answer, 12);
        assert_eq!(example_3_answer, 11);
    }

    #[test]
    fn test_delimiters() {
        let test_example = "(2,3) (4,3,2)";
        let answer = vec![vec![2, 3], vec![4, 3, 2]];

        assert_eq!(get_buttons(test_example), answer);

        let test_example = "[..##..#]";
        let answer = vec![false, false, true, true, false, false, true];

        assert_eq!(get_lights(test_example), answer);
    }

    #[test]
    fn print_matricies() {
        let input = read_input();

        let mut vec_buttons = Vec::new();
        for line in input.lines() {
            let buttons = get_buttons(line);
            vec_buttons.push(buttons);
        }

        for button_vec in vec_buttons {
            let matrix = Matrix::new(button_vec);
            println!("{}", matrix)
        }
    }
}
