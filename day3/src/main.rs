mod parser;

use std::{fs::File, io::{BufReader, Read}};

use anyhow::Result;
use parser::{Parser, Part1Processor, Part2Processor};

fn get_input() -> Result<String> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(buf)
}

// expected answer: 188192787
fn part_one() -> Result<()> {
    let input = get_input()?;

    let parser = Parser::<Part1Processor, _, _>::new(&input);

    let multiplications = parser.parse();
    let mut sum = 0;
    multiplications.into_iter().for_each(|(a, b)| sum += a * b);

    // println!("state: {:?}", parser);

    println!("part 1 total: {:?}", sum);

    Ok(())   
}

fn part_two() -> Result<()> {
    let input = get_input()?;

    let parser = Parser::<Part2Processor, _, _>::new(&input);

    let multiplications = parser.parse();
    let mut sum = 0;
    multiplications.into_iter().for_each(|(a, b)| sum += a * b);

    // println!("state: {:?}", parser);

    println!("part 2 total: {:?}", sum);

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