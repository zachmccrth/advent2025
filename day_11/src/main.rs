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

fn modified_part_1(graph: &NodeGraph, start_id: &str, end_id: Vec<&str>) -> Vec<u32> {
    let start_node = graph.get_node_from_id(start_id);

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

// algo in pseudo code (because I don't know what structure I need yet)
//
// basically, go through every node in dfs. if I detect a cycle (node goes back on itself, I
// can reset to the start of the cycle, as every node on that cycle has an infinite number of
// paths) and this node is stored as 0 (as well as every other node on the call stack)
//
// If I finish (get to out) then I need to check if fft and dac are on my call stack. If they
// are, then I know that the previous node in the call stack has at least one path. Once I
// finish out a node (visit all of its children), then I can store the number of paths it has
// to out... This then essentially becomes a new out with a number attached to it (the number
// of paths). So we know that essentially we create a type of node that has a value attached to
// it (the number of paths that reaching that node adds to the total)
//
// Additionally, there are nodes that have a certain number of paths that go through dac/fft,
// and some paths that don't. So really I have 4 values (paths throught fft, paths through dac,
// paths through both, paths through out.) All stored paths should have paths through out tho.
// If fft is in the downstream path, it may not be in the upstream path. Same for dac.
// Actually, this won't matter though. We just wait for 'dac' to collect all of its downstream
// nodes, then for 'fft' .
//
// So I need a structure that will make it clear when I have visited all downstream nodes, I
// could insert what is essentially a hook (clean up in the stack). That way I know that I have
// visited all of the nodes. Is probably the simplest way of doing it.

enum NodeOrCalling {
    VisitNode(usize),
    CallFinished(usize),
}

fn part_2_memoizied(input: &str) -> u64 {
    // if you build it, he will come?
    use self::NodeOrCalling::*;
    let nodes = parse_nodes(input);
    let graph = NodeGraph { nodes: nodes };

    let start_node = graph.get_node_from_id("svr");

    let mut stack = Vec::new();
    stack.push(NodeOrCalling::VisitNode(start_node.self_idx));

    let mut seen_fft = false;
    let mut seen_dac = false;

    let mut fully_visited_nodes = vec![-2; graph.nodes.len()];

    fully_visited_nodes[graph.get_node_from_id("out").self_idx] = 1;

    let mut paths: u64 = 0;
    while let Some(node_or_call) = stack.pop() {
        match node_or_call {
            VisitNode(node_idx) => {
                if fully_visited_nodes[node_idx] == -2 {
                    // we haven't seen this node yet, so add it to the stack as a call and all its
                    // child nodes as subsequent call (will visit them before we get back here)

                    // Add node call to stack,
                    stack.push(CallFinished(node_idx));
                    fully_visited_nodes[node_idx] = -1;

                    // and now its connections:
                    let node = &graph.get_node(node_idx);
                    if node.id == "fft" {
                        seen_fft = true;
                    } else if node.id == "dac" {
                        seen_dac = true;
                    }

                    for connection_idx in &node.connections_index {
                        stack.push(VisitNode(*connection_idx));
                    }
                } else if fully_visited_nodes[node_idx] == -1 {
                    // we have seen this node before (added it to the stack) so we assume a loop
                    // unwind the stack, marking all nodes in the loop as terminators:
                    loop {
                        let node_or_call = stack.pop().unwrap();

                        match node_or_call {
                            // node is off of a loop, so don't ever visit
                            VisitNode(to_remove_idx) => {
                                fully_visited_nodes[to_remove_idx] = 0;
                            }
                            CallFinished(to_remove_idx) => {
                                fully_visited_nodes[to_remove_idx] = 0;
                                if to_remove_idx == node_idx {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    // this is a terminator node (we don't keep exploring, already have the answer
                    // in 'fully visited')
                }
            }
            CallFinished(calling_node_idx) => {
                // once we finish all of the children of this node add the child paths to the
                // current node

                let current_node = graph.get_node(calling_node_idx);

                for child_node_idx in &current_node.connections_index {
                    fully_visited_nodes[current_node.self_idx] =
                        fully_visited_nodes[*child_node_idx];
                }
            }
        }
    }

    paths
}

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
