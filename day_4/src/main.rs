use std::fs::File;
use std::io::{Read, Result};

fn convolve(thing: &Vec<u8>, thing_row_size: usize) -> Vec<u8> {
    let mut new_vec: Vec<u8> = thing.clone();

    // This grabs the element above one row
    for i in thing_row_size..new_vec.len() {
        new_vec[i] += thing[i - thing_row_size];
    }
    // grabs the element below one row
    for i in 0..(new_vec.len() - thing_row_size) {
        new_vec[i] += thing[i + thing_row_size];
    }

    let mut newer_vec: Vec<u8> = new_vec.clone();
    let row_number = thing.len() / thing_row_size;

    // So now new_vec looks like the sum of (without boundaries):
    //
    //  . # .
    //  . # .
    //  . # .
    //
    // Where the indexed entry is at the center of the kernel

    //then grab the elements from the right (accumulated sum of the vertical right)
    // . . #
    // . . #
    // . . #
    for row_index in 0..row_number {
        for column_index in 0..thing_row_size - 1 {
            newer_vec[row_index * thing_row_size + column_index] +=
                new_vec[row_index * thing_row_size + column_index + 1]
        }
    }
    // and the elements from the left (accumulated sum of the vertical left)
    // # . .
    // # . .
    // # . .
    for row_index in 0..row_number {
        for column_index in 1..thing_row_size {
            newer_vec[row_index * thing_row_size + column_index] +=
                new_vec[row_index * thing_row_size + column_index - 1]
        }
    }

    newer_vec
}

fn count_rolls(original_map: &Vec<u8>, convolved_map: &Vec<u8>) -> u32 {
    // This gives the total sum
    //
    // # # #
    // # # #
    // # # #
    //
    // and every line has been counted. However, we want to ignore the states where the middle is
    // 0, and we want to subtract one from the other states as the central paper roll doesn't
    // count. So:
    //
    let mut count = 0;
    for i in 0..original_map.len() {
        if original_map[i] == 0 {
            continue;
        }
        if (convolved_map[i] - 1) < 4 {
            count += 1
        }
    }
    count
}

fn map_to_binary(contents: &str) -> (Vec<u8>, usize) {
    let roll: u8 = '@' as u8;

    let mut length_of_line = 0;

    let mut output: Vec<u8> = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        // line in bytes
        length_of_line = line.len();

        output.append(
            &mut line
                .as_bytes()
                .iter()
                .map(|&x| if x == roll { 1 } else { 0 })
                .collect(),
        );
    }

    (output, length_of_line)
}

fn part_1(contents: &str) -> u32 {
    let (thing, length_of_line) = map_to_binary(&contents);

    let convolution = convolve(&thing, length_of_line);

    let sum = count_rolls(&thing, &convolution);

    println!("Part 1 Solution {}", sum);

    sum
}

fn remove_rolls(map: &mut Vec<u8>, convolution: &Vec<u8>) -> u32 {
    let mut sum = 0;
    for i in 0..map.len() {
        if map[i] == 0 {
            continue;
        }
        if (convolution[i] - 1) < 4 {
            map[i] = 0;
            sum += 1;
        }
    }
    sum
}

fn part_2(contents: &str) -> u32 {
    let (mut thing, length_of_line) = map_to_binary(&contents);
    let mut sum = 0;
    loop {
        let convolution = convolve(&thing, length_of_line);

        let removed_rolls = remove_rolls(&mut thing, &convolution);

        if removed_rolls == 0 {
            break;
        } else {
            sum += removed_rolls;
        }
    }
    println!("Part 2 Solution {}", sum);
    sum
}

fn main() -> Result<()> {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt")?, &mut contents);

    part_1(&contents);

    part_2(&contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let contents = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let calc = part_1(&contents);
        assert_eq!(calc, 13);
    }

    #[test]
    fn test_example_2() {
        let contents = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let calc = part_2(&contents);
        assert_eq!(calc, 43);
    }
}
