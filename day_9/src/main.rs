use std::cmp::{max, min};
use std::{self, fs::File, io::Read, ops::RangeInclusive};

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

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn get_internal_ranges(
    point1: &Point,
    point2: &Point,
) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let (xmin, xmax) = (min(point1.x, point2.x) + 1, max(point1.x, point2.x) - 1);
    let (ymin, ymax) = (min(point1.y, point2.y) + 1, max(point1.y, point2.y) - 1);

    ((xmin..=xmax), (ymin..=ymax))
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

fn overlaps(rect_range: &RangeInclusive<i64>, edge_range: &RangeInclusive<i64>) -> bool {
    if rect_range.is_empty() {
        return false;
    }
    *rect_range.start() <= *edge_range.end() && (*edge_range.start() <= *rect_range.end())
}

#[derive(Debug)]
struct Edge {
    range: RangeInclusive<i64>,
    value: i64,
    direction: Direction,
}

impl Edge {
    fn new_edge(point1: &Point, point2: &Point) -> Edge {
        let x_diff = point1.x - point2.x;
        let range;
        let value;
        let direction;
        match x_diff {
            ..0 => {
                range = point1.x..=point2.x;
                value = point1.y;
                direction = Direction::X;
            }
            0 => {
                let y_diff = point1.y - point2.y;
                if y_diff < 0 {
                    range = point1.y..=point2.y
                } else {
                    range = point2.y..=point1.y
                }
                value = point1.x;
                direction = Direction::Y;
            }
            1.. => {
                range = point2.x..=point1.x;
                value = point1.y;
                direction = Direction::X;
            }
        }

        Edge {
            range: range,
            value: value,
            direction: direction,
        }
    }
}

fn check_edge(points: (&Point, &Point), edges: &Vec<Edge>) -> bool {
    let (point1, point2) = points;
    let (x_range, y_range) = get_internal_ranges(&point1, &point2);

    for edge in edges {
        match edge.direction {
            Direction::Y => {
                if x_range.contains(&edge.value) {
                    if overlaps(&y_range, &edge.range) {
                        return false;
                    }
                }
            }
            Direction::X => {
                if y_range.contains(&edge.value) {
                    if overlaps(&x_range, &edge.range) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn part_2(input: &str) -> i64 {
    let tuple_points = extract_tuples(&input);

    let mut points = Vec::new();
    let mut edges = Vec::new();

    let last_tuple_point = tuple_points.iter().last().unwrap();
    let mut last_point = Point {
        x: last_tuple_point.0,
        y: last_tuple_point.1,
    };
    for tuple_point in tuple_points {
        let point = Point {
            x: tuple_point.0,
            y: tuple_point.1,
        };
        edges.push(Edge::new_edge(&last_point, &point));
        points.push(point.clone());
        last_point = point;
    }

    let mut max_area = 0;

    for i in 0..points.len() {
        let point1 = &points[i];
        for j in 0..points.len() {
            let point2 = &points[j];
            let area = ((point2.x - point1.x).abs() + 1) * ((point2.y - point1.y).abs() + 1);
            if area > max_area {
                if check_edge((point1, point2), &edges) {
                    max_area = area;
                    println!(
                        "Points {:?}, {:?} passed inspection, new max area of {}",
                        point1, point2, max_area
                    )
                }
            }
        }
    }

    println!("{:?}", points);
    println!("{:?}", edges);
    max_area
}

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Solution Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input);

    println!("Solution Part 2: {}", part_2_answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = String::from("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n");

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 50);
    }

    #[test]
    fn test_example2() {
        let input = String::from("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n");

        let part_2_answer = part_2(&input);

        assert_eq!(part_2_answer, 24);
    }
}
