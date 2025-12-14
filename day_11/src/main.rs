use std::collections::{HashMap, VecDeque};
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
    fn get_start(self: &Self) -> &Node {
        self.nodes.iter().find(|node| node.id == "you").unwrap()
    }

    fn get_node(self: &Self, index: usize) -> &Node {
        &self.nodes[index]
    }
}

fn part_1(input: &str) -> u32 {
    let nodes = parse_nodes(input);
    let graph = NodeGraph { nodes: nodes };

    let start_node = graph.get_start();

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

fn modified_part_1(graph: &NodeGraph, start_id: &str, end_id: Vec<&str>) -> Vec<u32> {
    let start_node = graph.nodes.iter().find(|node| node.id == start_id).unwrap();

    let mut to_visit = VecDeque::new();

    to_visit.push_back(start_node);

    let mut paths = vec![0; end_id.len()];

    loop {
        let current_node = to_visit.pop_front();

        match current_node {
            Some(node) => {
                if end_id.contains(&node.id.as_str()) {
                    let index = (0..end_id.len())
                        .find(|i| end_id[*i] == node.id.as_str())
                        .unwrap();
                    paths[index] += 1;
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

fn reverse_graph(old_graph: &NodeGraph) -> NodeGraph {
    let mut new_nodes = old_graph.nodes.clone();

    // reset the connection in the new graph
    for new_node in &mut new_nodes {
        new_node.connections_index.clear();
    }

    // for every node in the old graph, add itself as a connection to its downstream nodes (reverse
    // the graph)
    for old_node in &old_graph.nodes {
        for connection_idx in &old_node.connections_index {
            new_nodes[*connection_idx]
                .connections_index
                .push(old_node.self_idx);
        }
    }

    NodeGraph { nodes: new_nodes }
}

fn part_2(input: &str) -> u32 {
    let nodes = parse_nodes(input);
    let graph = NodeGraph { nodes: nodes };

    let start_node = graph.nodes.iter().find(|node| node.id == "svr").unwrap();

    let mut to_visit = VecDeque::new();

    // just going to wrap in a tuple now (fft, dac)
    // jk we are going to need to prune this better
    //
    // we consider three types of paths from srv -> end in fft, end in dac, end in out
    //
    // then from fft we consider dac and out
    // and from dac we consider fft and out
    //
    // technically we can only have one of the three types, because if fft -> dac and dac -> fft
    // that is a loop and the number of paths is infinite
    //
    // so which comes first? and then we can simplify
    to_visit.push_back((start_node, false, false));

    let mut paths = 0;
    loop {
        let current_node = to_visit.pop_front();

        match current_node {
            Some((node, fft, dac)) => {
                if node.is_out() {
                    if fft & dac {
                        paths += 1;
                    }
                } else {
                    // Push all of the connecting nodes in the current node
                    for new_node in node
                        .connections_index
                        .iter()
                        .map(|index| graph.get_node(*index))
                    {
                        to_visit.push_back((
                            new_node,
                            (node.id == "fft") | fft,
                            (node.id == "dac") | dac,
                        ));
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    paths
}


// I am now guessing he sneakily slapped a cycle in here. Will need to find a way to detect
// (switch to DFS, backtracking...)
fn part_2_cycle_detection(input: &str) -> u32 {
    let nodes = parse_nodes(input);
    let graph = NodeGraph { nodes: nodes };

    let start_node = graph.nodes.iter().find(|node| node.id == "svr").unwrap();

    let mut to_visit = VecDeque::new();

    // wil also do DFS here for better cycle detection
    // to build a "call stack" with backtracking, store calling node for traversal
    to_visit.push_back((start_node, "start"));
    // and create a last_visited to compare to calling node:
    let mut call_stack = Vec::new();

    let mut paths = 0;
    loop {
        // DFS:
        let optional_node = to_visit.pop_back();

        match optional_node {
            Some((current_node, calling_node_id)) => {
                // reset the call stack to the calling node
                while !call_stack.is_empty() && *call_stack.last().unwrap() != calling_node_id {
                    call_stack.pop();
                }

                // detect a loop
                if call_stack.contains(&current_node.id.as_str()) {
                    // we want to continue checking the remaining paths of the current loop while
                    // not going back down the path we came. so just don't add this
                    // node to the next up stack
                    //
                    // continue;
                    //
                    // actually, we technically can terminate all nodes added downstream of this
                    // node because we know that any node downstream of this has infinite possible paths (and won't be
                    // the puzzle answer)
                    //
                    // so, we remove nodes until we get to nodes that had
                    // &current_node.id.as_str() as their calling node, and we remove those too
                    let mut removed_nodes = VecDeque::new();
                    loop {
                        let to_remove = to_visit.pop_back();
                        match to_remove {
                            Some(node_call) => {
                                if node_call.1 == current_node.id.as_str() {
                                    break;
                                }
                                removed_nodes.push_front(node_call);
                            }
                            None => {
                                // the looping node didn't add any other nodes, we add them all back
                                to_visit.append(&mut removed_nodes);
                                break;
                            }
                        }
                    }
                    loop {
                        let to_remove = to_visit.pop_back();
                        match to_remove {
                            Some(node_call) => {
                                if node_call.1 != current_node.id.as_str() {
                                    to_visit.push_back(node_call);
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }

                    continue;
                }

                if current_node.id == "out" {
                    if call_stack.contains(&"dac") & call_stack.contains(&"fft") {
                        paths += 1;
                    }
                } else {
                    // Push all of the connecting nodes in the current node
                    // add current node as calling node for backtracing
                    for new_node in current_node
                        .connections_index
                        .iter()
                        .map(|index| graph.get_node(*index))
                    {
                        to_visit.push_back((new_node, &current_node.id));
                    }

                    call_stack.push(&current_node.id.as_str());

                    assert!(call_stack.len() < 1000);
                }
            }
            None => {
                break;
            }
        }
    }

    paths
}


fn part_2

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let mod_part = part_2_cycle_detection(&input);

    println!("Solution: {:?}", mod_part);
    // let part_2_answer = part_2(&input);
    // println!("Part 2: {}", part_2_answer);
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

        // add a loop between fft and dac (ccc: eel) eel: ell, ell: eel                      <
        let input_loop = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee eel\neel: ell\nell: eel\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out\n";

        let part_2_answer = part_2_cycle_detection(&input_loop);

        assert_eq!(part_2_answer, 2);
    }
}
