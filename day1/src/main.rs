use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}};

fn get_input() -> Result<(Vec<i64>, Vec<i64>), std::io::Error> {
    let input = File::open("input.txt")?;
    let lines = BufReader::new(input).lines();

    let lists: (Vec<i64>, Vec<i64>) = lines.into_iter().map(|s| {
        let s = s.unwrap();
        let trimmed = s.trim();

        let [a, b]: [i64; 2] = trimmed.split("   ").map(|t| t.parse().unwrap()).collect::<Vec<i64>>().try_into().unwrap();

        (a, b)
    }).unzip();

    Ok(lists)
}

fn part_one() -> Result<(), std::io::Error> {
    let (mut list1, mut list2) = get_input()?;

    list1.sort();
    list2.sort();

    let mut dist = 0;
    list1.into_iter().zip(list2.into_iter()).for_each(|(a, b)| dist += (a - b).abs());

    println!("total distance: {}", dist);

    Ok(())
}

fn part_two() -> Result<(), std::io::Error> {
    let (list1, list2) = get_input()?;

    let mut numbers = HashMap::new();

    list1.into_iter().for_each(|n| {numbers.insert(n, 0i64);});

    list2.into_iter().for_each(|n| {
        match numbers.get_mut(&n) {
            Some(count) => *count += 1,
            None => ()
        }
    });

    let mut score = 0;
    numbers.into_iter().for_each(|(k, v)| score += k * v);

    println!("similarity score: {}", score);

    Ok(())
}

fn main() {
    let r1 = part_one();
    let r2 = part_two();

    r1.unwrap();
    r2.unwrap();
}