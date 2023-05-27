use crate::db;

// NOTE
// Run this test with flag --test-threads=1
// Example: cargo test -- --test-threads=1

mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn read(){
        let conn = &Connection::open("sqlite.db").expect("Error DB connection");
        let append = db::show_notes(&conn);
        match append {
            Ok(_) => assert_eq!(true, true),
            Err(e) => assert_eq!(true, false, "Помилка {:?}", e),
        };
    }

    #[test]
    fn write() {
        let conn = &Connection::open("sqlite.db").expect("Error DB connection");
        let w = "World".to_string();
        let append = db::append_note(&conn, &w);
        match append {
            Ok(_) => assert_eq!(true, true),
            Err(e) => assert_eq!(true, false, "Помилка {:?}", e),
        };
    }

    #[test]
    fn correct_write(){
        let conn = &Connection::open("sqlite.db").expect("Error DB connection");
        let mut stmt = conn.prepare("SELECT message FROM notes ORDER BY id DESC").unwrap();
        let row = stmt.query_row([], |row| row.get::<_, String>(0)).unwrap();
        assert_eq!(row, "World".to_string());
    }
}