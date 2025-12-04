use std::fs::File;
use std::io::{Read, Result};

fn get_nearby_wrap_checking(
    index: usize,
    length: usize,
) -> (Option<usize>, Option<usize>, Option<usize>) {
    let current_row = index / length;
    let minus = if (index - 1) / length == current_row {
        Some(index - 1)
    } else {
        None
    };
    let plus = if (index + 1) / length == current_row {
        Some(index + 1)
    } else {
        None
    };

    return (minus, Some(index), plus);
}

fn count_accessible_part_1(vector: &Vec<u8>, length: usize) {
    let mut count = 0;
    for i in 0..vector.len() {
        if vector[i] == 1 {}
    }
}

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

fn part_1(contents: &str) -> u32 {
    let roll: u8 = '@' as u8;

    let mut length_of_line = 0;

    let mut thing: Vec<u8> = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        // line in bytes
        length_of_line = line.len();

        thing.append(
            &mut line
                .as_bytes()
                .iter()
                .map(|&x| if x == roll { 1 } else { 0 })
                .collect(),
        );
    }

    let convolution = convolve(&thing, length_of_line);

    let sum = count_rolls(&thing, &convolution);

    println!("Part 1 Solution {}", sum);

    sum
}

fn main() -> Result<()> {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open("input.txt")?, &mut contents);

    part_1(&contents);

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
}
