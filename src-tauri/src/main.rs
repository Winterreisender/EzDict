#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use rusqlite::{Connection};
use serde::{Deserialize, Serialize};
// Prevents additional console window on Windows in release, DO NOT REMOVE!!

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
fn query(word :&str) -> QueryResult {
    dbg!(word);

    let conn = Connection::open("../asset/default.dict.db").unwrap();
    let mut query = conn.prepare("SELECT description FROM dict WHERE word=?1;").unwrap();
    let descriptions = query.query_map([word], |row|{
        let r :String = row.get(0)?;
        Ok(r)
    });

    let r = match descriptions {
        Ok(mut x) => x.next().unwrap_or(Ok(String::from("Nothing Found"))).unwrap_or("Nothing Found 2".to_string()),
        Err(err) => err.to_string()
    };


    QueryResult { descriptions: r }
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
