use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    fs::File,
    io::Read,
    ops::Deref,
    rc::Rc,
};

fn read_input() -> String {
    let mut input = String::new();
    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut input);
    input
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    distance_squared: i64,
    smaller_index: usize,
    larger_index: usize,
}

fn distance_squared(node1: (i64, i64, i64), node2: (i64, i64, i64)) -> i64 {
    (node1.0 - node2.0) ^ 2 + (node1.1 - node2.1) ^ 2 + (node1.2 - node2.2) ^ 2
}

fn parse_node(string: &str) -> (i64, i64, i64) {
    let mut string_numbers = string.split(',');

    (
        string_numbers.next().unwrap().parse().unwrap(),
        string_numbers.next().unwrap().parse().unwrap(),
        string_numbers.next().unwrap().parse().unwrap(),
    )
}

fn part_1(input: &str) -> u64 {
    let mut visited = Vec::new();

    let mut edges = Vec::new();
    for (new_index, line) in input.lines().enumerate() {
        let new_node = parse_node(&line);
        for (existing_index, existing_node) in visited.iter().enumerate() {
            edges.push(Edge {
                distance_squared: distance_squared(*existing_node, new_node),
                smaller_index: existing_index,
                larger_index: new_index,
            })
        }
        visited.push(new_node);
    }

    edges.sort();
    edges.truncate(1000);

    0
}

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Part 1 Solution: {}", part_1_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        assert_eq!(part_1(&input), 40);
    }
}
