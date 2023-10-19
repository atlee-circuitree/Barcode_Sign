
/*
SQL Logic goal
- take a row
- find the next entry that matches the same student id
- if action of found entry is i (in) stop there
- if action is o (out) record the duration and write to another table
*/
use std::io;
use std::error::Error;
use rusqlite::{Connection, Result};

fn main() -> Result<(), Box<dyn Error>> {

    let connection = Connection::open("test.db")?;

    let query_init = "CREATE TABLE IF NOT EXISTS logs (id INT, act CHAR, time TEXT)";
    connection.execute(query_init, ())?;
    
    loop {
        let input: String = strin();
        if input == "X" {
            break
        };

        let id:String = String::from(&input[1..]);
        let id_int = match id.parse::<i32>() {
            Ok(i) => i,
            _ => 0
        };
        
        let mut act: String = String::from(&input[..1]);
        if act.len() > 1 {
            act = "x".to_string();
        }

        let query: String = format!("INSERT INTO logs VALUES ({id_int}, '{act}', datetime('now'));");
        connection.execute(&query, ())?;
    }
    Ok(())
}

fn strin() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = String::from(buffer.trim());
    buffer
}