use rusqlite::{Connection, Result, named_params};

#[derive(Debug)]
struct List {
    id: u64,
    message: String,
    done: u8,
}

// Показує всі нотатки
pub fn show_notes(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, message, done FROM notes ORDER BY id")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let list = List{
            id: row.get(0)?,
            message: row.get(1)?,
            done: row.get(2)?,
        };
        if list.done == 0 {
            println!("{} {}", list.id, list.message);
        } else {
            println!("{} {}", list.id, "✅ ".to_owned() + &list.message);
        }
    }
    Ok(())
}

// Додає нову нотатку
pub fn append_note(conn: &Connection, note: &String) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO notes (message, done) VALUES (:message, :done)")?;
    stmt.execute(named_params! {":message": &note.to_string(), ":done": 0})?;
    println!("Нотатка додана");
    Ok(())
}

// Помічає нотатку як виконану
pub fn mark_done(conn: &Connection, index: &u16) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE notes SET done = :done WHERE id = :id")?;
    let affected_rows = stmt.execute(named_params! {":done": 1, ":id": &index})?;
    if affected_rows > 0 {
        println!("Нотатка {} відмічена", &index);
    } else {
        println!("Нотатки з індексом {} немає", &index);
    }
    Ok(())
}

// Видаляє нотатки позначені як виконані
fn clear_done(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM notes WHERE done = 1",
        [],
    )?;
    println!("Відмічені нотатки видалені");
    Ok(())
}

// Видаляє всі нотатки
fn clear_all(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM notes",
        [],
    )?;
    println!("Всі нотатки видалені");
    Ok(())
}

// Видаляє одну нотатку по індексу
pub fn command_remove(conn: &Connection, index: &u16) -> Result<()> {
    let mut stmt = conn.prepare("DELETE FROM notes WHERE id = :id")?;
    let affected_rows = stmt.execute(named_params! {":id": &index})?;
    if affected_rows > 0 {
        println!("Нотатка {} видалена ", &index);
    } else {
        println!("Нотатки з індексом {} немає ", &index);
    }
    Ok(())
}

// Команда на очистку нотаток
pub fn command_clear(conn: &Connection, all: &bool) -> Result<()> {
    if *all == false {
        clear_done(&conn)
    } else {
        clear_all(&conn)
    }
}
