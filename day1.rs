use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read(path: &str) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}

fn part1(vec: Vec<i64>) -> i64 {
    let mut num_deeper: i64 = 0;
    let mut previous_depth: i64 = 0;

    for num in vec.iter() {
        if previous_depth > 0 && *num > previous_depth {
            num_deeper += 1;
        }
        previous_depth = *num;
    }

    return num_deeper;
}

fn part2(vec: Vec<i64>) -> i64 {
    let mut num_deeper: i64 = 0;
    let mut cache: i64 = 0;
    let mut current: i64;

    for i in 0..vec.len() {
        current = 0;

        for j in 0..3 {
            if i + j < vec.len() {
                current += vec[i + j];
            }
        }

        if cache != 0 && current > cache {
            num_deeper += 1;
        }
        cache = current;
    }

    return num_deeper;
}

fn main() {
    if let Ok(vec) = read("./depths.txt") {
        println!("Part 1: {}", part1(vec.clone()));
        println!("Part 2: {}", part2(vec.clone()));
    } else {
        println!("Error reading file");
    }
}
