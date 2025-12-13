use std::char::UNICODE_VERSION;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn read_input() -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut contents);

    contents
}

fn parse_nodes(input: &str) -> Vec<Node> {
    let mut output = Vec::new();

    let mut id_to_index = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let (id, _) = line.split_once(':').unwrap();
        id_to_index.insert(id, i);
    }

    for (i, line) in input.lines().enumerate() {
        let (id, nodes) = line.split_once(':').unwrap();

        let mut connections = Vec::new();

        for node in nodes.trim().split_whitespace() {
            connections.push(*id_to_index.get(node).unwrap());
        }

        output.push(Node {
            id: String::from(id),
            self_idx: i,
            connections: connections,
        })
    }
    output
}

struct Node {
    id: String,
    self_idx: usize,
    connections: Vec<usize>,
}

fn part_1(nodes: Vec<Node>) {}

fn main() {
    let input = read_input();
    let nodes = parse_nodes(&input);

    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out\n";
    }
}
