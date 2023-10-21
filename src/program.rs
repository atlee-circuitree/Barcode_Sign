use rusqlite::{Connection, Result};
use std::error::Error;
use std::io;
#[derive(Debug, Clone)]
pub struct Entry {
    id: i32,
    act: String,
    time: String,
}

impl Entry {
    pub fn print(self) {
        println!("ID {}, ACTION {}, TIME {}", self.id, self.act, self.time)
    }

}
pub struct Session {
    id: i32,
    in_time: String,
    out_time: String,
}
impl Session {
    pub fn print(self) {
        println!("ID {}, IN TIME {}, OUT TIME {}", self.id, self.in_time, self.out_time);
    }
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
    let session = find_logout(&entries, 0);
    match session {
        Some(a) =>  a.print(),
        None => println!("Error"),
    }
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

fn find_logout(table: &Vec<Entry>, session_start: usize) -> Option<Session>{
    let start_entry = table[session_start].clone();
    let future_entries = &table[(session_start + 1)..];
    for e in future_entries {
        let entry = e.clone();
        if entry.id == start_entry.id {
            match entry.act.as_str() {
                "i" => {
                    let s = Session{
                        id: start_entry.id,
                        in_time: start_entry.time.clone(),
                        out_time: entry.time.clone(),
                    };
                    return Some(s)
                },
                _ => {
                    let s = Session{
                        id: start_entry.id,
                        in_time: start_entry.time.clone(),
                        out_time: start_entry.time.clone(),
                    };
                    return Some(s)
                }
            }
        }
    }
    return None
}