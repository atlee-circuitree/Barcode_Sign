
/*
SQL Logic goal
- take a row
- find the next entry that matches the same student id
- if action of found entry is i (in) stop there
- if action is o (out) record the duration and write to another table
*/
use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let connection = sqlite::open("test.db")?;

    let query_init = "CREATE TABLE IF NOT EXISTS logs (id INT, act CHAR, time TEXT)";
    connection.execute(query_init)?;
    
    loop {
        let input = strin();
        if input == "X" {
            break
        };

        let id: &str = &input[1..];
        let act: &str = &input[..1];
        let query = format!("INSERT INTO logs VALUES ({id}, '{act}', datetime('now'));");
        connection.execute(query)?;
    }
    Ok(())
}

fn strin() -> String {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer = String::from(buffer.trim());
    buffer
}