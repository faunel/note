
mod db;
mod cli;

#[cfg(test)]
mod test;

use rusqlite::Connection;

fn main() {
    let conn = Connection::open("./../../sqlite.db").expect("Error DB connection");
    cli::run(&conn);
}


