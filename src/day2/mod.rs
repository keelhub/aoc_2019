use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn run() {
    let values = read_csv(Path::new("src/day2/input"));
    part1(&values);
    part2(&values);
}

fn read_csv<P: AsRef<Path>>(filename: P) -> Vec<i32> {
    let display = filename.as_ref().display();
    let mut file = match File::open(&filename) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    let v: Vec<i32> = s
        .trim_end()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    v
}

fn part1(values: &Vec<i32>) {
    let mut v = values.to_owned();
    v[1] = 12;
    v[2] = 2;
    computer(&mut v);
    println!("{}", v[0]);
}

fn part2(values: &Vec<i32>) {
    'outer: for i in 0..99 {
        'inner: for j in 0..99 {
            let mut v = values.to_owned();
            v[1] = i;
            v[2] = j;
            computer(&mut v);
            if v[0] == 19690720 {
                println!("{}, {}: {}", i, j, 100*i + j);
                break 'outer;
            }
        }
    }
}

fn computer(v: &mut Vec<i32>) {
    for i in (0..v.len()).step_by(4) {
        match v[i] {
            1 => {
                let (p1, p2, out) = (v[i + 1] as usize, v[i + 2] as usize, v[i + 3] as usize);
                v[out] = v[p1] + v[p2];
            }
            2 => {
                let (p1, p2, out) = (v[i + 1] as usize, v[i + 2] as usize, v[i + 3] as usize);
                v[out] = v[p1] * v[p2];
            }
            99 => break,
            _ => println!("Say what?: {}", v[i]),
        }
    }
}
