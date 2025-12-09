use std::{fs::File, io::Read};

fn read_input() -> String {
    let mut input = String::new();
    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut input);
    input
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Edge {
    distance_squared: i64,
    smaller_index: usize,
    larger_index: usize,
}

fn distance_squared(node1: (i64, i64, i64), node2: (i64, i64, i64)) -> i64 {
    (node1.0 - node2.0).pow(2) + (node1.1 - node2.1).pow(2) + (node1.2 - node2.2).pow(2)
}

fn parse_node(string: &str) -> (i64, i64, i64) {
    let mut string_numbers = string.split(',');

    (
        string_numbers.next().unwrap().parse().unwrap(),
        string_numbers.next().unwrap().parse().unwrap(),
        string_numbers.next().unwrap().parse().unwrap(),
    )
}

#[derive(Debug)]
enum NodeType {
    Node(usize),
    Root(u64),
}

struct Node {
    self_idx: usize,
    nodetype: NodeType,
}

struct UnionFind {
    arena: Vec<Node>,
}

impl UnionFind {
    fn new(capacity: usize) -> Self {
        let mut arena = Vec::new();
        for i in 0..capacity {
            arena.push(Node {
                self_idx: i,
                nodetype: NodeType::Root(1),
            })
        }
        UnionFind { arena: arena }
    }

    fn node(self: &Self, query: usize) -> &Node {
        &self.arena[query]
    }

    fn find(self: &Self, query: usize) -> usize {
        let mut node = self.node(query);

        loop {
            match node.nodetype {
                NodeType::Root(_) => {
                    return node.self_idx;
                }
                NodeType::Node(parent_idx) => {
                    node = self.node(parent_idx);
                }
            }
        }
    }

    fn merge(self: &mut Self, node1: usize, node2: usize) -> &Node {
        let root1_idx = self.find(node1);
        let root2_idx = self.find(node2);
        match (
            &self.arena[root1_idx].nodetype,
            &self.arena[root2_idx].nodetype,
        ) {
            (NodeType::Root(size1), NodeType::Root(size2)) => {
                self.arena[root1_idx] = Node {
                    self_idx: root1_idx,
                    nodetype: NodeType::Root(size1 + size2),
                };
                self.arena[root2_idx] = Node {
                    self_idx: root2_idx,
                    nodetype: NodeType::Node(root1_idx),
                };

                return self.node(root1_idx); // returns new root
            }
            other => panic!("Should both be Roots! Found {:?}", other),
        }
    }
}

fn extract_edges(input: &str) -> Vec<Edge> {
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

    edges
}

fn part_1(input: &str, connections: u32) -> u64 {
    let mut edges = extract_edges(&input);
    edges.truncate(connections as usize);

    let mut union_find = UnionFind::new(input.lines().count());

    for edge in edges {
        if union_find.find(edge.smaller_index) != union_find.find(edge.larger_index) {
            union_find.merge(edge.smaller_index, edge.larger_index);
        }
    }

    let mut sizes = Vec::new();
    for node in union_find.arena {
        match node.nodetype {
            NodeType::Root(size) => sizes.push(size),
            _ => {}
        }
    }

    sizes.sort();
    sizes.reverse();

    sizes.get(0).unwrap_or(&1) * sizes.get(1).unwrap_or(&1) * sizes.get(2).unwrap_or(&1)
}

fn part_2(input: &str) -> i64 {
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

    let num_nodes = input.lines().count();

    let mut union_find = UnionFind::new(num_nodes);

    let edges_iter = edges.iter();

    let mut last_edge = &Edge {
        distance_squared: 0,
        smaller_index: 0,
        larger_index: 0,
    };

    for edge in edges_iter {
        if union_find.find(edge.smaller_index) != union_find.find(edge.larger_index) {
            let new_root = union_find.merge(edge.smaller_index, edge.larger_index);
            match new_root.nodetype {
                NodeType::Root(size) => {
                    if size as usize == num_nodes {
                        last_edge = edge;
                        break;
                    }
                }
                NodeType::Node(_) => {
                    panic!("Should be root!");
                }
            }
        }
    }

    let node1 = visited.get(last_edge.smaller_index).unwrap();
    let node2 = visited.get(last_edge.larger_index).unwrap();

    node1.0 * node2.0
}

fn main() {
    let input = read_input();
    let connections = 1000;

    let part_1_answer = part_1(&input, connections);

    println!("Part 1 Solution: {}", part_1_answer);

    let part_2_answer = part_2(&input);

    println!("Part 2 Solution: {}", part_2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let connections = 10;
        assert_eq!(part_1(&input, connections), 40);
    }
}
