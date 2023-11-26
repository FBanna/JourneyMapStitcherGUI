//// TO DO ////

/// launcher ///

// MC, Prism, Multi MC



// Journey map data
// Xairo map data (cant remember name)
// mabye MC world data (in far future)

use std::{fs, path::PathBuf};
use std::path::Path;
use directories::{BaseDirs, UserDirs, ProjectDirs};
use tauri::api::path::data_dir;


pub fn mc_data() -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut multi_player: Vec<PathBuf> = Vec::new();
    let mut single_player: Vec<PathBuf> = Vec::new();

    if let Some(data_dir) = BaseDirs::new() {

        let multi_player_path = &data_dir.data_dir().join(".minecraft\\journeymap\\data\\mp");

        if multi_player_path.exists() {
            let paths = fs::read_dir(multi_player_path).unwrap();

            for path in paths {
                
                multi_player.push(path.unwrap().path().clone());

            }

        } else {

            println!("does not exist!")

        }
    }

    if let Some(data_dir) = BaseDirs::new() {

        let single_player_path = &data_dir.data_dir().join(".minecraft\\journeymap\\data\\sp");

        if single_player_path.exists() {
            let paths = fs::read_dir(single_player_path).unwrap();

            for path in paths {
                
                single_player.push(path.unwrap().path().clone());

            }

        } else {

            println!("does not exist!")
            
        }
    }


    
    return(multi_player, single_player)
}