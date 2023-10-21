use rusqlite::{Connection, Result};
use std::error::Error;
use std::io;
#[derive(Debug, Clone)]
pub struct Entry {
    id: i32,
    act: i32,
    time: String,
}

impl Entry {
    pub fn _print(self) {
        println!("ID {}, ACTION {}, TIME {}", self.id, self.act, self.time)
    }

}

#[derive(Debug, Clone)]
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

//enum for tracking program state. probably wont be using this any time soon
/*
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
*/

pub fn log() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open("test.db")?;

    let query_init = "CREATE TABLE IF NOT EXISTS logs (id INT, act BIT, time TEXT)";
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
        

        let act = &input[..1];

        let act = match act {
            "i" => 1,
            "o" => 0,
            _ => 0,
        };
        let query: String =
            format!("INSERT INTO logs VALUES ({id_int}, {act}, datetime('now'));");
        connection.execute(&query, ())?;
    }
    Ok(())
}

pub fn _analyze() -> Result<(), Box<dyn Error>> {
    
    let conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("SELECT id, act, time FROM logs")?;
    /*
    I have no idea what this code is doing, I adapted it from the documentation
    in the inevitable case I need to read from the db again, just replace the struct with whatever is suitable
    */
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
    

    let mut sessions: Vec<Session> = Vec::new();

    for i in 0..(entries.len()-1) {
        if entries[i].act == 1 {
            match find_logout(&entries, i) {
                Some(a) => sessions.push(a),
                None => (),
            }
        }
        
    }
    for i in sessions {
        i.print();
    }
    Ok(())
}


//This will be the function used to search for specific users and stats eventually
pub fn _display() {
    todo!();
}

//this might be used to export to some sort of file that can then be used for apis
//keyword might since idk how async works
pub fn _export() {
    todo!();
}

// just a shorthand for taking input. probably will replace with a CLI library later
fn strin() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer = String::from(buffer.trim());
    buffer
}

//iterates through the list of entries, finding the next one with the same id
//then makes a struct with the time of that entry if it is a logout, or with the same time if it is a login
//functionally recreates that monstrosity of a cell equation from the google sheet
fn find_logout(table: &Vec<Entry>, session_start: usize) -> Option<Session>{
    let start_entry = table[session_start].clone();
    let future_entries = &table[(session_start + 1)..];
    for entry in future_entries {
        let entry = entry.clone();
        entry._print();
    }
    println!();
    for e in future_entries {
        let entry = e.clone();
        if entry.id == start_entry.id {
            match entry.act{
                0 => {
                    let s = Session{
                        id: start_entry.id,
                        in_time: start_entry.time.clone(),
                        out_time: entry.time.clone(),
                    };
                    return Some(s)
                },
                1 => {
                    let s = Session{
                        id: start_entry.id,
                        in_time: start_entry.time.clone(),
                        out_time: start_entry.time.clone(),
                    };
                    return Some(s)
                }
                _ => return None,
            }
        }
    }
    return None
}