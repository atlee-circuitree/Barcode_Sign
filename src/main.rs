/*
SQL Logic goal

- take a row
- find the next entry that matches the same student id
- if action of found entry is i (in) stop there
- if action is o (out) record the duration and write to another table
*/

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let connection = sqlite::open("test.db")?;
    let query = "
        INSERT INTO Logs VALUES ('i', 47553, datetime('now'));
        INSERT INTO Logs VALUES ('o', 47553, datetime('now'));
    ";
    connection.execute(query)?; 
    Ok(())
}
