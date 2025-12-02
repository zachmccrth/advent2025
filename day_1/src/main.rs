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

    println!("{}", zeros);

    Ok(())
}
