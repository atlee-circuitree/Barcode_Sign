
use rusqlite::{Connection, Result};
use std::error::Error;
use std::io;
use cliclack::*;
use qrcode_generator::QrCodeEcc;



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
    pub fn _print(self) {
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
    conn.execute("DROP TABLE IF EXISTS sessions;", ())?;
    conn.execute("CREATE TABLE IF NOT EXISTS sessions (id INT, timein TEXT, timeout TEXT, duration TEXT);", ())?;
    
    for session in sessions {
        
        let query = format!("INSERT INTO sessions VALUES ({}, '{}', '{}', strftime('%s','{}') - strftime('%s','{}'));", 
        session.id , session.in_time, session.out_time, session.out_time, session.in_time);
        conn.execute(&query, ()).expect("sql 2 error");
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

pub struct User {
    first_name: String,
    last_name: String,
    id: i32,
    //role: String,
    //leadership: Option<i32>,
}
pub fn _register() -> Result<(), Box<dyn Error>> {
    //let qr_code = qr_code::QrCode::new(b"Hello").unwrap();
    //let bmp = qr_code.to_bmp();
    //bmp.write(std::fs::File::create("test.bmp").unwrap()).unwrap();

    let conn = Connection::open("test.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS users (firstName TEXT, lastName TEXT, id INT )", ())?;
    intro("User registration process")?;
    loop {
        
        let f_name: String = input("First name: ").interact()?;
        let l_name: String = input("Last name name: ").interact()?;
        let id1: i32 = input("Id number:").interact()?;
        let user: User = User {
            first_name: f_name,
            last_name: l_name,
            id: id1,
        };
        let query = format!("INSERT INTO users VALUES ('{}', '{}', {});", user.first_name, user.last_name, user.id);
        conn.execute(&query, ())?;
        let path = format!("/img/i{}.png", user.id);
        let text = format!("i{}", user.id);
        qrcode_generator::to_png_to_file(text, QrCodeEcc::Low, 1024, path).unwrap();
        break;
    }
    outro("Registration done")?;
    Ok(())
}