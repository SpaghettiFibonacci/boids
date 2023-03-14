// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::Rng;

use std::vec;

use lazy_static::lazy_static;
use tauri::async_runtime::Mutex;
pub mod models;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

lazy_static! {
    static ref BIRDS: Mutex<Vec<models::bird::Bird>> = Mutex::new(vec![]);
}

#[tauri::command]
async fn make_birds() {
    let mut birds = BIRDS.lock().await;
    for _i in 0..100 {
        // push bird with random position
        birds.push(models::bird::Bird::new(
            rand::random::<f32>() * 500.0,
            rand::random::<f32>() * 500.0,
            rand::thread_rng().gen_range(-0.5..0.5),
            rand::thread_rng().gen_range(-0.5..0.5),
        ));
    }
}

#[tauri::command]
async fn update_birds() -> Vec<Vec<f32>> {
    // println!("update_bird called");
    let mut bird_positions = Vec::new();
    let mut birds = BIRDS.lock().await;
    let mut birddies = birds.clone();
    for bird in birds.iter_mut() {
        // println!("bird: {:?}", bird);
        bird.run(&birddies, (0.0, 0.0), 500.0, 500.0);
        bird_positions.push(vec![bird.x, bird.y]);
    }

    bird_positions
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, update_birds, make_birds])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
