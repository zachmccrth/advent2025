use std::fs::File;
use std::io::{self, Read};

fn extract_step(step: &str) -> i32 {
    let mut characters = step.chars();
    let direction_char = characters.next().expect("Must have an initial character!");
    let sign: i32 = if direction_char == 'L' { -1 } else { 1 };
    let number_string: String = characters.collect();
    let magnitude: i32 = number_string
        .parse()
        .expect("Could not parse number_string");
    return magnitude * sign;
}

fn does_op_cross_zero(state: i32, op: i32) -> bool {
    let remainder_op = op % 100;
    // tricky special case, we need to check if we are already at boundary
    if state == 0 {
        return false; // Is impossible to loop back around by definition of remainder
    }
    // checks to see if state has moved more than 50 away from center; that is, beyond 0 or 100
    if (state + remainder_op - 50).abs() >= 50 {
        true
    } else {
        false
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();

    let mut state = 50;
    let mut zeros = 0;

    for line in lines {
        let scalar_operator = extract_step(line);
        state = (state + scalar_operator) % 100;
        if state == 0 {
            zeros += 1
        }
    }

    println!("Part 1 Solution: {}", zeros);

    let lines = contents.lines();

    let mut state = 50;
    let mut zeros = 0;

    for line in lines {
        let scalar_operator = extract_step(line);
        let full_rotations = (scalar_operator / 100).abs();
        zeros += full_rotations;
        if does_op_cross_zero(state, scalar_operator) {
            zeros += 1
        }
        state = ((state + scalar_operator) % 100 + 100) % 100;
    }

    println!("Part 2 Solution: {}", zeros);

    Ok(())
}
