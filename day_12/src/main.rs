use std::fs::File;
use std::io::Read;

fn read_input(filename: &str) -> String {
    let mut contents = String::new();

    let _ = File::read_to_string(&mut File::open(filename).unwrap(), &mut contents);

    contents
}

#[derive(Debug)]
struct Shape {
    index: usize,
    filled: Vec<Vec<bool>>,
}

#[derive(Debug)]
struct Region {
    area: (usize, usize),
    shapes: Vec<u32>,
}

fn parse_input() -> (Vec<Shape>, Vec<Region>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let input = read_input("shapes.txt");
    let mut index: i32 = -1;
    let mut filled: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        let option = line.split_once(':');
        match option {
            Some((index_str, _)) => {
                if index != -1 {
                    shapes.push(Shape {
                        index: index as usize,
                        filled: filled.clone(),
                    });
                    filled.clear();
                }
                println!("Index {}", index_str);
                index = index_str.parse().unwrap();
            }
            None => {
                filled.push(line.chars().map(|char| char == '#').collect());
            }
        }
    }

    let input = read_input("regions.txt");
    for line in input.lines() {
        let (area, shape_nums) = line.split_once(':').unwrap();
        let (height, width) = area.split_once('x').unwrap();

        let shape_vec: Vec<u32> = shape_nums
            .split_whitespace()
            .map(|string| string.parse().unwrap())
            .collect();

        regions.push(Region {
            area: (height.parse().unwrap(), width.parse().unwrap()),
            shapes: shape_vec,
        });
    }
    println!("Shapes: {:?}, Regions {:?}", shapes, regions);
    (shapes, regions)
}

fn trivial_check(regions: &Vec<Region>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut trivial_fail = Vec::new();
    let mut trivial_pass = Vec::new();
    let mut complicated = Vec::new();
    for (i, region) in regions.iter().enumerate() {
        let area = region.area.0 * region.area.1;

        if region.shapes.iter().sum::<u32>() * 7 >= area as u32 {
            trivial_fail.push(i);
        } else if region.shapes.iter().sum::<u32>()
            <= ((region.area.0 / 3) * (region.area.1 / 3)) as u32
        {
            trivial_pass.push(i);
        } else {
            complicated.push(i);
        }
    }

    (trivial_fail, trivial_pass, complicated)
}

fn main() {
    let (shapes, regions) = parse_input();

    let (fail, pass, complicated) = trivial_check(&regions);

    println!(
        "Fail: {}, Pass: {}, Complicated: {}",
        fail.len(),
        pass.len(),
        complicated.len()
    );
}
