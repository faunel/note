
use crate::db;

use clap::{Parser, Subcommand};
use rusqlite::Connection;

pub fn run (conn: &Connection) {
    let cli = Cli::parse();

    let r = match &cli.command {
        Some(Commands::Add { note }) => db::append_note(&conn, &note),
        Some(Commands::Remove { index }) => db::command_remove(conn, &index),
        Some(Commands::Done { index }) => db::mark_done(&&conn, &index),
        Some(Commands::Clear { all }) => db::command_clear(&conn, &all),
        _ => db::show_notes(&conn),
    };

    if let Err(e) = r {
        println!("{:?}", e);
    }
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { // Команда додавання нотатки
        #[clap(value_parser)]
        note: String, // Текст нотатки
    },
    /// remove a sticky note
    Remove { // Команда видалення нотатки
        #[clap(value_parser)]
        index: u16, // Індекс нотатки
    },
    Done { // Команда виставлення прапорця нотатки
        #[clap(value_parser)]
        index: u16, // Індекс нотатки
    },
    Clear { // Команда видалення нотатки яка відмічена або всіх якщо переданий аргумент `-a`
        #[clap(short, long, action)]
        all: bool, // Якщо true видаляемо всі нотатки
    },
}