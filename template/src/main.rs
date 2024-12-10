use anyhow::Result;

fn part_one() -> Result<()> {
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