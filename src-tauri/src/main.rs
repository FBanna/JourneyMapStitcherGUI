// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stitch(x1: i32, y1: i32, x2:i32, y2:i32, style: String){
    println!("stitch called with {} {} {} {}", x1, y1, x2, y2);

    if style == "span"{
        let mut xsize: i32 = i32::abs((x1-x2)/512);
        let mut ysize: i32 = i32::abs((y1-y2)/512);


        //if x1 < x2{
        //    startxtile = 
        //}
    }
}

fn creation(startingx: i32, startingy: i32, width: i32, height: i32, out: String){
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stitch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
