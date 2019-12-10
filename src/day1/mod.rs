use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("{}", part1());
    println!("{}", part2());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1() -> u64 {
    // Create a path to the desired file
    let path = Path::new("src/day1/input");

    let mut fuel: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                fuel += ((ip.parse::<f64>().unwrap() / 3.).floor() - 2.) as u64;
            }
        }
    }
    fuel
}

fn part2() -> u64 {
    // Create a path to the desired file
    let path = Path::new("src/day1/input");

    let mut fuel: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mass = ip.parse::<i64>().unwrap();
                fuel += (get_fuel_mass(mass) - mass) as u64
            }
        }
    }
    fuel
}

fn get_fuel_mass(initial_mass: i64) -> i64 {
    let fuel = (initial_mass / 3) - 2;

    if fuel < 0 {
        initial_mass
    } else {
        initial_mass + get_fuel_mass(fuel)
    }
}