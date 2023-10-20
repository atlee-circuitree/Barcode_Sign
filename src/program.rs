use rusqlite::{Connection, Result};
use std::error::Error;
use std::io;

struct Entry {
    id: i32,
    act: String,
    time: String,
}
enum _ProgramState {
    Logging,
    Analyzing,
    Displaying,
    Exporting,
    Idle,
}

fn _state_change() {
    todo!();
}

pub fn log() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open("test.db")?;

    let query_init = "CREATE TABLE IF NOT EXISTS logs (id INT, act CHAR, time TEXT)";
    connection.execute(query_init, ())?;

    loop {
        let input: String = strin();
        if input == "X" {
            break;
        };

        let id: String = String::from(&input[1..]);
        let id_int = match id.parse::<i32>() {
            Ok(i) => i,
            _ => 0,
        };

        let act: String = String::from(&input[..1]);

        let query: String =
            format!("INSERT INTO logs VALUES ({id_int}, '{act}', datetime('now'));");
        connection.execute(&query, ())?;
    }
    Ok(())
}

pub fn _analyze() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("SELECT id, act, time FROM logs")?;
    let logs_iter = stmt.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            act: row.get(1)?,
            time: row.get(2)?,
        })
    })?;
    let mut entries = vec![];
    for entry in logs_iter {
        entries.push(entry?)
    }
    let entry_one = &entries[0]; 
    println!("{}, {}, {}", entry_one.act, entry_one.time, entry_one.id);
    Ok(())
}

pub fn _display() {
    todo!();
}

pub fn _export() {
    todo!();
}

fn strin() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = String::from(buffer.trim());
    buffer
}
