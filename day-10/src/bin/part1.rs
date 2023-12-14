use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // build a map of the pipes & record start location
    let mut start_point: Point = Point::default();
    let mut pipe_map: HashMap<Point, char> = HashMap::new();

    for (y, line) in input.lines().enumerate().collect::<Vec<(usize, &str)>>() {
        for (x, c) in line.chars().enumerate().collect::<Vec<(usize, char)>>() {
            pipe_map.insert(Point::new(x, y), c);
            if c == 'S' {
                start_point = Point::new(x, y)
            }
        }
    }

    // find a valid direction of travel from start location
    let mut start_heading: Directions = Directions::North;
    for &dir in [
        &Directions::North,
        &Directions::South,
        &Directions::East,
        &Directions::West,
    ] {
        if start_point.check_dir(dir, &pipe_map) {
            start_heading = dir;
            break;
        }
    }

    println!("Start @ {:?}, Heading {:?}", start_point, start_heading);

    // make first step
    let mut steps: usize = 0;
    let mut curent_loc: Point = start_point.clone();

    curent_loc.move_dir(start_heading);
    let mut current_heading = dbg!(find_heading(start_heading, pipe_map[&curent_loc]).unwrap());
    steps += 1;

    println!(
        "Step: {} - {} - {:?} >> {:?}: Heading: {:?}",
        steps, pipe_map[&curent_loc], start_point, curent_loc, current_heading
    );

    while start_point != curent_loc {
        curent_loc.move_dir(current_heading);
        steps += 1;

        if pipe_map[&curent_loc] != 'S' {
            current_heading = find_heading(current_heading, pipe_map[&curent_loc]).unwrap();
        }

        println!(
            "Step: {} - {} - {:?} >> {:?}: Heading: {:?}",
            steps, pipe_map[&curent_loc], start_point, curent_loc, current_heading
        );

        if steps > pipe_map.len() {
            panic!("Too many steps.");
        }
    }

    let mid_point = steps / 2_usize;
    mid_point.to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Directions {
    North,
    South,
    East,
    West,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    fn move_dir(&mut self, dir: Directions) {
        match dir {
            Directions::North => self.y -= 1,
            Directions::South => self.y += 1,
            Directions::East => self.x += 1,
            Directions::West => self.x -= 1,
        }
    }

    fn check_dir(&self, dir: Directions, map: &HashMap<Point, char>) -> bool {
        let mut cpoint: Point = self.clone();

        match dir {
            Directions::North => {
                cpoint.y -= 1;
                match find_heading(Directions::North, map[&cpoint]) {
                    None => false,
                    _ => true,
                }
            }
            Directions::South => {
                cpoint.y += 1;
                match find_heading(Directions::South, map[&cpoint]) {
                    None => false,
                    _ => true,
                }
            }
            Directions::East => {
                cpoint.x += 1;
                match find_heading(Directions::East, map[&cpoint]) {
                    None => false,
                    _ => true,
                }
            }
            Directions::West => {
                cpoint.x -= 1;
                match find_heading(Directions::West, map[&cpoint]) {
                    None => false,
                    _ => true,
                }
            }
        }
    }
}

fn find_heading(heading: Directions, pipe_section: char) -> Option<Directions> {
    match pipe_section {
        '|' => match heading {
            Directions::North => Some(Directions::North),
            Directions::South => Some(Directions::South),
            _ => None,
        },
        '-' => match heading {
            Directions::East => Some(Directions::East),
            Directions::West => Some(Directions::West),
            _ => None,
        },
        'L' => match heading {
            Directions::South => Some(Directions::East),
            Directions::West => Some(Directions::North),
            _ => None,
        },
        'J' => match heading {
            Directions::South => Some(Directions::West),
            Directions::East => Some(Directions::North),
            _ => None,
        },
        '7' => match heading {
            Directions::East => Some(Directions::South),
            Directions::North => Some(Directions::West),
            _ => None,
        },
        'F' => match heading {
            Directions::West => Some(Directions::South),
            Directions::North => Some(Directions::East),
            _ => None,
        },
        '.' | 'S' => None,
        _ => panic!("Pipe section not recognised {pipe_section}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, "8".to_string());

        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, "4".to_string());
    }
}
