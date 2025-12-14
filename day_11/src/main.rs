use std::collections::{HashMap, HashSet, VecDeque};
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

    id_to_index.insert("out", id_to_index.len());

    for (i, line) in input.lines().enumerate() {
        let (id, nodes) = line.split_once(':').unwrap();

        let mut connections = Vec::new();

        for node in nodes.trim().split_whitespace() {
            connections.push(
                *id_to_index
                    .get(node)
                    .expect(&format!("String should have contained a node {}", node)),
            );
        }

        output.push(Node {
            id: String::from(id),
            self_idx: i,
            connections_index: connections,
        })
    }

    output.push(Node {
        id: String::from("out"),
        self_idx: output.len(),
        connections_index: Vec::new(),
    });

    output
}

#[derive(Clone, PartialEq)]
struct Node {
    id: String,
    self_idx: usize,
    connections_index: Vec<usize>,
}

impl Node {
    fn is_out(self: &Self) -> bool {
        self.id == "out"
    }
}

struct NodeGraph {
    nodes: Vec<Node>,
}

impl NodeGraph {
    fn get_node(self: &Self, index: usize) -> &Node {
        &self.nodes[index]
    }

    fn get_node_from_id(self: &Self, id: &str) -> &Node {
        self.nodes.iter().find(|node| node.id == id).unwrap()
    }
}

fn part_1(input: &str) -> u32 {
    let nodes = parse_nodes(input);
    let graph = NodeGraph { nodes: nodes };

    let start_node = graph.get_node_from_id("you");

    let mut to_visit = VecDeque::new();

    to_visit.push_back(start_node);

    let mut paths = 0;
    loop {
        let current_node = to_visit.pop_front();

        match current_node {
            Some(node) => {
                if node.is_out() {
                    paths += 1;
                } else {
                    // Push all of the connecting nodes in the current node
                    node.connections_index
                        .iter()
                        .map(|index| graph.get_node(*index))
                        .for_each(|node| to_visit.push_back(node));
                }
            }
            None => {
                break;
            }
        }
    }

    paths
}

fn recursive_part_2(
    memo: &mut HashMap<(usize, bool, bool), u64>,
    fft: bool,
    dac: bool,
    to_visit: usize,
    graph: &NodeGraph,
) -> u64 {
    let current_node = graph.get_node(to_visit);

    if current_node.id == "out" {
        return if fft && dac { 1 } else { 0 };
    }

    let key = (to_visit, fft, dac);
    if let Some(&cached) = memo.get(&(to_visit, fft, dac)) {
        return cached;
    }

    let mut paths = 0;
    let fft_next = fft || current_node.id == "fft";
    let dac_next = dac || current_node.id == "dac";

    for &child_idx in &current_node.connections_index {
        paths += recursive_part_2(memo, fft_next, dac_next, child_idx, graph);
    }

    memo.insert(key, paths);
    paths
}

fn part_2(input: &str) -> u64 {
    let mut memo = HashMap::new();

    let graph = NodeGraph {
        nodes: parse_nodes(&input),
    };
    let start = graph.get_node_from_id("svr");

    return recursive_part_2(&mut memo, false, false, start.self_idx, &graph);
}

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input);

    println!("Solution: {:?}", part_2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out\n";

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 5);

        let input = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out\n";

        let part_2_answer = part_2(&input);

        assert_eq!(part_2_answer, 2);
    }
}
