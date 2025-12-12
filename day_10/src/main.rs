use std::fs::File;
use std::io::Read;

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
        }<
    }

    output
}

fn get_numeric(input_line: &str, delimiters: (char, char)) -> Vec<usize> {
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
        let buttons = get_numeric(line, ('(', ')'));
        let batteries = get_numeric(line, ('{', '}'));

        output.push(Problem {
            lights: lights,
            buttons: buttons,
            batteries: batteries,
        })
    }

    output
}

struct Problem {
    lights: Vec<bool>,
    buttons: Vec<usize>,
    batteries: Vec<usize>,
}


fn solve_problem_1(problem: &Problem) -> u32 {


    0
}


fn part_1(input: &str) -> u64 {

    let problems = parse(&input);




    0
}

fn main() {
    let input = read_input();
    let part_1_answer = part_1(&input);
    println!("Part 1 Solution {}", part_1_answer);
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn test_delimiters() {
        let test_example = "(2,3)";
        let answer = vec![2, 3];

        assert_eq!(get_numeric(test_example, ('(', ')')), answer);

        let test_example = "[..##..#]";
        let answer = vec![false, false, true, true, false, false, true];

        assert_eq!(get_lights(test_example), answer);
    }
}
