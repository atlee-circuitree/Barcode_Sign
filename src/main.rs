
/*
SQL Logic goal
- take a row
- find the next entry that matches the same student id
- if action of found entry is i (in) stop there
- if action is o (out) record the duration and write to another table
*/
use std::error::Error;
mod program;

fn main() -> Result<(), Box<dyn Error>> {
    program::log()?;
    Ok(())
}

