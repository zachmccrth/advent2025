use std::fs::File;
use std::io::Read;

const SPLITTER: u8 = '^' as u8;
const SOURCE: char = 'S';

fn read_input() -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt").unwrap(), &mut contents);

    contents
}

struct OrderedPushSet {
    values: Vec<usize>,
}

impl OrderedPushSet {
    fn push(self: &mut Self, value: usize) {
        // this is a bit complicated, but we only ever need to check the last value here.
        // We will always push in order because either we have a conflict where a previous split
        // "occupied" the current index we are pushing to, or we have a conflict because we had a
        // previously occupied state (from a straight path) and we are splitting into it, but there
        // we also push the lowest split value first (so the compare still works).
        //
        //
        // Note that this is dramatically overcomplicated, so if this doesn't work in part 2, just
        // use a hashset
        match self.values.last() {
            Some(last_value) => {
                if value == *last_value {
                    return;
                } else {
                    self.values.push(value)
                }
            }
            None => self.values.push(value),
        }
    }

    fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.values.iter()
    }
}

struct OrderedPushSetPart2 {
    values: Vec<(usize, u64)>,
}

impl OrderedPushSetPart2 {
    fn push(self: &mut Self, next_value: (usize, u64)) {
        // slightly modified versions that keeps track of the number of paths that lead to this
        // index
        match self.values.last_mut() {
            Some((last_value_index, current_number_of_paths)) => {
                if next_value.0 == *last_value_index {
                    *current_number_of_paths += next_value.1;
                } else {
                    self.values.push(next_value)
                }
            }
            None => self.values.push(next_value),
        }
    }

    fn iter(&self) -> std::slice::Iter<'_, (usize, u64)> {
        self.values.iter()
    }
}

fn part_1(input: &str) -> u32 {
    let mut input_iter = input.lines();
    // initialize tachyons based on first line
    let first_line = input_iter.next().unwrap();
    let source_index = first_line.find(SOURCE).unwrap();

    let mut current_tachyons_index = OrderedPushSet {
        values: vec![source_index],
    };

    let mut output = 0;
    for line in input_iter {
        let mut next_tachyon_line = OrderedPushSet { values: Vec::new() };
        for tachyon_index in current_tachyons_index.iter() {
            if line.as_bytes()[*tachyon_index] == SPLITTER {
                next_tachyon_line.push(*tachyon_index - 1);
                next_tachyon_line.push(*tachyon_index + 1);
                output += 1
            } else {
                next_tachyon_line.push(*tachyon_index);
            }
        }

        current_tachyons_index = next_tachyon_line;
    }

    output
}

fn part_2(input: &str) -> u64 {
    let mut input_iter = input.lines();
    // initialize tachyons based on first line
    let first_line = input_iter.next().unwrap();
    let source_index = first_line.find(SOURCE).unwrap();

    let mut current_tachyons_index = OrderedPushSetPart2 {
        values: vec![(source_index, 1)],
    };

    for line in input_iter {
        let mut next_tachyon_line = OrderedPushSetPart2 { values: Vec::new() };
        for (tachyon_index, number_of_paths) in current_tachyons_index.iter() {
            if line.as_bytes()[*tachyon_index] == SPLITTER {
                next_tachyon_line.push((*tachyon_index - 1, *number_of_paths));
                next_tachyon_line.push((*tachyon_index + 1, *number_of_paths));
            } else {
                next_tachyon_line.push((*tachyon_index, *number_of_paths));
            }
        }

        current_tachyons_index = next_tachyon_line;
    }

    current_tachyons_index.values.iter().map(|v| v.1).sum()
}

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Part 1 Solution: {}", part_1_answer);

    let part_2_answer = part_2(&input);

    println!("Part 2 Solution: {}", part_2_answer);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let input = String::from(
            ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n",
        );

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 21);
    }

    #[test]
    fn test_2() {
        let input = String::from(
            ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n",
        );

        let part_2_answer = part_2(&input);

        assert_eq!(part_2_answer, 40);
    }
}
