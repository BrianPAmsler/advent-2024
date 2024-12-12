use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::Result;

fn get_input() -> Result<Vec<Vec<char>>> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut input = Vec::new();

    lines.into_iter().for_each(|line| {
        let line = line.unwrap();
        let chars = line.chars().map(|c| c).collect();

        input.push(chars);
    });

    Ok(input)
}

fn get_char(input: &Vec<Vec<char>>, r: isize, c: isize) -> Option<char> {
    if r < 0 || r >= input.len() as isize || c < 0 || c >= input[0].len() as isize {
        return None;
    }

    Some(input[r as usize][c as usize])
}

fn get_line<const N: usize>(input: &Vec<Vec<char>>, r: usize, c: usize, dr: isize, dc: isize) -> [Option<char>; N] {
    let mut arr = [None; N];

    if dr == 0 && dc == 0 {
        return arr;
    }

    for i in 0..N {
        arr[i] = get_char(input, r as isize + dr * i as isize, c as isize + dc * i as isize);
    }

    arr
}

fn find_xmas(input: &Vec<Vec<char>>, r: usize, c: usize) -> u32 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {

            let line = get_line(input, r, c, dr, dc);

            if line == [Some('X'), Some('M'), Some('A'), Some('S')] {
                count += 1;
            }
        }
    }

    count
}

fn part_one() -> Result<()> {
    let input = get_input()?;

    let mut total = 0;
    for r in 0..input.len() {
        for c in 0..input[0].len() {
            if get_char(&input, r as isize, c as isize) == Some('X') {
                total += find_xmas(&input, r, c);
            }
        }
    }

    println!("part one: {}", total);

    Ok(())   
}

fn part_two() -> Result<()> {
    Ok(())
}

fn main() {
    let r1 = part_one();
    let r2 = part_two();

    match r1 {
        Err(e) => eprintln!("Part one error: {:?}", e.backtrace()),
        Ok(_) => (),
    }

    match r2 {
        Err(e) => eprintln!("Part two error: {:?}", e.backtrace()),
        Ok(_) => (),
    }
}