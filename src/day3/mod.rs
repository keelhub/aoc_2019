use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let x = read_csv(Path::new("src/day3/input"));
    part1(x);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::with_capacity(2);
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                data.push(ip.split(',').map(|x| x.to_string()).collect());
            }
        }
    }
    data
}

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn x_min(&self) -> i64 {
        std::cmp::min(self.start.x, self.end.x)
    }

    fn x_max(&self) -> i64 {
        std::cmp::max(self.start.x, self.end.x)
    }

    fn y_min(&self) -> i64 {
        std::cmp::min(self.start.y, self.end.y)
    }

    fn y_max(&self) -> i64 {
        std::cmp::max(self.start.y, self.end.y)
    }
}

fn part1(wires: Vec<Vec<String>>) {
    let mut wires1: Vec<Line> = Vec::<Line>::new();
    let mut wires2: Vec<Line> = Vec::<Line>::new();
    get_points(&wires[0], &mut wires1);
    get_points(&wires[1], &mut wires2);

    let intersects = get_intersections(&wires1, &wires2);
    for i in intersects.iter() {
        println!("{:?}", i);
    }
    let mut minimum: Vec<usize> = intersects
        .iter()
        .map(|x| x.manhattan_distance())
        .collect::<Vec<_>>();
    minimum.sort();
    for i in minimum.iter() {
        println!("Distance: {:?}", i);
    }
}

fn get_points(path: &Vec<String>, lines: &mut Vec<Line>) {
    let mut p = Point { x: 0, y: 0 };

    for i in path.iter().enumerate() {
        let (d, vals) = i.1.split_at(1);
        let val: i64 = vals.parse().unwrap();
        let start = p.clone();
        match d {
            "L" => p.x -= val,
            "R" => p.x += val,
            "D" => p.y -= val,
            "U" => p.y += val,
            _ => panic!("What?"),
        }
        lines.push(Line {
            start: start,
            end: p.clone(),
        })
    }
}

fn get_intersections(w1: &Vec<Line>, w2: &Vec<Line>) -> Vec<Point> {
    let mut intersects = Vec::<Point>::new();
    for l1 in w1.iter() {
        for l2 in w2.iter() {
            if (l1.x_min() >= l2.x_min() && l1.x_min() <= l2.x_max())
                && (l2.y_min() >= l1.y_min() && l2.y_min() <= l1.y_max())
            {
                intersects.push(Point::new(l1.x_min(), l2.y_min()))
            }
        }
    }
    intersects
}
