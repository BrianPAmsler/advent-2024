mod parser;

use std::{fs::File, io::{BufReader, Read}};

use anyhow::Result;
use parser::Parser;

fn get_input() -> Result<String> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(buf)
}

fn part_one() -> Result<()> {
    let input = get_input()?;

    let mut parser = Parser::new(&input);

    let mut sum = 0;
    while parser.has_next() {
        match parser.parse_next() {
            Some((a, b)) => sum += a * b,
            None => ()
        }
        // println!("parser state: {:?}", &parser);
    }

    // println!("state: {:?}", parser);

    println!("total: {:?}", sum);

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