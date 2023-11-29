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
use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;
use paths_as_strings;


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


pub fn get_last_world() -> (PathBuf){
    let mut path_to_world: PathBuf = PathBuf::new();

    let file_path = Path::new("worldSave.txt");

    //;
    let mut path_string: String = String::from("");
    match File::open(file_path) {
       Ok(file) => path_string = fs::read_to_string(file_path).expect("Unable to read file"),
       Err(error) => println!("unable to open file {}\n{}",file_path.display(),error),
    };

    path_to_world = paths_as_strings::decode_path(&path_string).unwrap();




    return path_to_world;
}

pub fn store_last_world() {

}