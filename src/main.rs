fn main() {
    let connection = sqlite::open("test.db").unwrap();

    let query = "
        INSERT INTO Logs VALUES ('i', 47553, datetime('now'));
        INSERT INTO Logs VALUES ('o', 47553, datetime('now'));
    ";
    connection.execute(query).unwrap(); 
}
