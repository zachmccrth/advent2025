use std::cmp::{max, min};
use std::{self, fs::File, io::Read};

fn read_input() -> String {
    let mut input = String::new();
    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut input);
    input
}

fn extract_tuples(input: &str) -> Vec<(i64, i64)> {
    let mut output: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let (string1, string2) = line.split_once(',').unwrap();
        output.push((string1.parse().unwrap(), string2.parse().unwrap()));
    }

    output
}

// fuck it, we loop
fn part_1(input: &str) -> u64 {
    let mut max_area: u64 = 0;

    let points = extract_tuples(&input);

    for i in 0..points.len() {
        let point1 = points[i];
        for j in 0..points.len() {
            let point2 = points[j];
            let area = ((point2.0 - point1.0).abs() + 1) * ((point2.1 - point1.1).abs() + 1);

            if area as u64 > max_area {
                max_area = area as u64;
            }
        }
    }

    max_area as u64
}

fn check_edges(
    points: ((i64, i64), (i64, i64)),
    xedges: Vec<(i64, i64, i64)>,
    yedges: Vec<(i64, i64, i64)>,
) -> bool {
    let (point1, point2) = points;

    let x_range = min(point1.0, point2.0)..=max(point1.0, point2.0);

    for xedge in xedges {}

    let y_range = min(point1.1, point2.1)..=max(point1.1, point2.1);
    true
}

// we loop again (why not?)
fn part_2(input: &str) -> u64 {
    let points = extract_tuples(&input);

    0
}

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Solution Part 1: {}", part_1_answer)
}

#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn test_example1() {
        let input = String::from("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n");

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 50);
    }
}
