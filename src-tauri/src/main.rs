#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{path::{Path, PathBuf}, io::{self, ErrorKind}, error::Error};

use rusqlite::{Connection};
use serde::{Deserialize, Serialize};
// Prevents additional console window on Windows in release, DO NOT REMOVE!!

const DB_PATH :&str = "../asset/";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct QueryResult {
    descriptions :String
}

#[tauri::command]
fn query(word :&str) -> Vec<QueryResult> {
    dbg!(word);

    let mut result :Vec<QueryResult> = vec![];

    for db_path in list_dictdb(Path::new(DB_PATH)).unwrap() {
        let conn = Connection::open(db_path).unwrap();
        let mut query = conn.prepare("SELECT description FROM dict WHERE word=?1;").unwrap();
        let descriptions = query.query_map([word], |row|{
            let r :String = row.get(0)?;
            Ok(r)
        });
        let r = match descriptions {
            Ok(mut x) => x.next().unwrap_or(Ok(String::from("Nothing Found"))).unwrap_or("Nothing Found 2".to_string()),
            Err(err) => err.to_string()
        };

        result.push(QueryResult { descriptions: r })
    }
    
    result
}

// WTF!
// Human readability is more important than ANYTHING in Software Enginneering!
fn list_dictdb(dict_dir :&Path) -> Result<Vec<PathBuf>, io::Error>{
    if !dict_dir.is_dir() {
        return Err(io::Error::new(ErrorKind::Other, format!("{:?} is not a directory", dict_dir)));
    }

    let r = std::fs::read_dir(dict_dir)?
        .map(|it| it.map(|it| it.path()))
        .filter(|it| it.as_ref().map(|x| x.is_file()).unwrap_or(false) )
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(r)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
