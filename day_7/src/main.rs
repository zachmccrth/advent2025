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

fn main() {
    let input = read_input();

    let part_1_answer = part_1(&input);

    println!("Part 1 Solution: {}", part_1_answer);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let input = String::from(
            ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n",
        );

        let part_1_answer = part_1(&input);

        assert_eq!(part_1_answer, 21);
    }
}
