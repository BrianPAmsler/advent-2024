use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::Result;

fn get_input() -> Result<Vec<Vec<i64>>> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();

    let input = lines.into_iter().map(|line| {
        let line = line.unwrap();

        let report: Vec<i64> = line.trim().split(' ').map(|s| s.parse().unwrap()).collect();

        report
    }).collect();

    Ok(input)
}

fn part_one() -> Result<()> {
    let reports = get_input()?;

    let mut safe_count = 0;
    for report in reports {
        let mut safe = true;
        let mut last_sign = -2;
        let mut last_value = report[0];

        for value in &report[1..] {
            let value = value.clone();
            let diff = value - last_value;

            let abs = diff.abs();
            let sign = if diff == 0 {
                0
            } else {
                diff / abs
            };

            if !((last_sign == -2 || (last_sign == sign && sign != 0)) && (abs <= 3 && abs != 0)) {
                safe = false;
                break;
            }

            last_sign = sign;
            last_value = value;
        }

        if safe {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);

    Ok(())
}

fn part_two() -> Result<()> {
    let reports = get_input()?;

    let mut safe_count = 0;
    for report in reports {
        let mut bad = 0;
        let mut last_sign = -2;
        let mut last_value = report[0];

        for value in &report[1..] {
            let value = value.clone();
            let diff = value - last_value;

            let abs = diff.abs();
            let sign = if diff == 0 {
                0
            } else {
                diff / abs
            };

            if !((last_sign == -2 || last_sign == sign) && (abs <= 3 && abs != 0)) {
                bad += 1;
            } else {
                last_sign = sign;
            }

            last_value = value;
        }

        if bad <= 1 {
            safe_count += 1;
        }
    }

    println!("Safe count (with tolerance): {}", safe_count);

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
