use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;

enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn new(raw_dir: char) -> Result<Direction, String> {
        match raw_dir {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            _   => Err(format!("invalid direction: {}!", raw_dir))
        }
    }
}

struct Path {
    direction: Direction,
    distance: i32
}

impl Path {
    fn new(command: &str) -> Path {
        let direction = command.chars().next().unwrap();
        let direction = Direction::new(direction).unwrap();
        let distance = &command[1..];
        let distance = distance.parse().unwrap();

        Path { direction, distance }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Wire {
    points: HashSet<Point>,
    steps: HashMap<Point, i32>
}

fn parse_input() -> Vec<Wire> {
    let f = File::open("./input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut wires = Vec::new();

    for line in reader.lines() {
        if let Ok(input) = line {
            wires.push(parse_wire(&input));
        }
    }

    wires
}

fn parse_wire(line: &str) -> Wire {
    let paths: Vec<Path> = line.split(',').map(|x| Path::new(x)).collect();

    let mut points = HashSet::new();
    let mut steps_for_points = HashMap::new();

    let mut steps = 0;
    let mut current_position = Point { x: 0, y: 0 };
    for path in paths {
        for _ in 0..path.distance {
            steps += 1;
            let point: Point;

            match path.direction {
                Direction::Up => {
                    point = Point { 
                        x: current_position.x,
                        y: current_position.y + 1,
                    };
                },
                Direction::Down => {
                    point = Point { 
                        x: current_position.x,
                        y: current_position.y - 1,
                    };
                },
                Direction::Left => {
                    point = Point { 
                        x: current_position.x - 1,
                        y: current_position.y,
                    };
                },
                Direction::Right => {
                    point = Point { 
                        x: current_position.x + 1,
                        y: current_position.y,
                    };
                }
            }

            current_position = point;
            points.insert(current_position.clone());
            if !steps_for_points.contains_key(&current_position) {
                steps_for_points.insert(current_position.clone(), steps);
            }
        }
    }

    Wire { points, steps: steps_for_points }
}

fn find_closest_intersection(wire1: &HashSet<Point>, wire2: &HashSet<Point>) -> Result<i32, String> {
    let mut intersection_points: Vec<&Point> = Vec::new();

    for point in wire1.intersection(wire2) {
        intersection_points.push(point);
    }

    if intersection_points.is_empty() {
        return Err(String::from("No intersections between wire 1 and wire 2!"));
    }

    let origin = Point { x: 0, y: 0 };
    intersection_points.sort_by(|a, b| {
        let manhattan_a = origin.manhattan_distance(a);
        let manhattan_b = origin.manhattan_distance(b);
        if manhattan_a < manhattan_b {
            Ordering::Less
        } else if manhattan_a == manhattan_b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    Ok(origin.manhattan_distance(intersection_points[0]))
}

fn find_least_amount_of_steps(wire1: &Wire, wire2: &Wire) -> Result<i32, String> {
    let mut intersection_points: Vec<i32> = Vec::new();

    for point in wire1.points.intersection(&wire2.points) {
        let step1 = wire1.steps.get(point).unwrap();
        let step2 = wire2.steps.get(point).unwrap();

        intersection_points.push(step1 + step2);
    }

    if intersection_points.is_empty() {
        return Err(String::from("No intersections between wire 1 and wire 2!"));
    }

    intersection_points.sort();
    Ok(intersection_points[0])
}

fn main() {
    let wires = parse_input();

    let min_dist = find_closest_intersection(&wires[0].points, &wires[1].points).unwrap();
    let min_steps = find_least_amount_of_steps(&wires[0], &wires[1]).unwrap();

    println!("Minimum distance: {}", min_dist);
    println!("Minimum steps: {}", min_steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_input() {
        let line1 = "R8,U5,L5,D3";
        let line2 = "U7,R6,D4,L4";
        
        let wire1 = parse_wire(&line1);
        let wire2 = parse_wire(&line2);

        let min_dist = find_closest_intersection(&wire1.points, &wire2.points).unwrap();
        assert_eq!(min_dist, 6);

        let min_steps = find_least_amount_of_steps(&wire1, &wire2).unwrap();
        assert_eq!(min_steps, 30);
    }

    #[test]
    fn test_sample_input_1() {
        let line1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let line2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        
        let wire1 = parse_wire(&line1);
        let wire2 = parse_wire(&line2);

        let min_dist = find_closest_intersection(&wire1.points, &wire2.points).unwrap();
        assert_eq!(min_dist, 159);

        let min_steps = find_least_amount_of_steps(&wire1, &wire2).unwrap();
        assert_eq!(min_steps, 610);
    }

    #[test]
    fn test_sample_input_2() {
        let line1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let line2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        let wire1 = parse_wire(&line1);
        let wire2 = parse_wire(&line2);

        let min_dist = find_closest_intersection(&wire1.points, &wire2.points).unwrap();
        assert_eq!(min_dist, 135);
        
        let min_steps = find_least_amount_of_steps(&wire1, &wire2).unwrap();
        assert_eq!(min_steps, 410);
    }
}