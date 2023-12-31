//// TO DO ////

/// launcher /// 

// MC, Prism, Multi MC

// CHECK current Dir



// Journey map data
// Xairo map data (cant remember name)
// mabye MC world data (in far future)

use std::result;
use std::{fs, path::PathBuf};
use std::path::Path;
use directories::{BaseDirs, UserDirs, ProjectDirs};
use tauri::api::path::data_dir;
use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;
use paths_as_strings;


pub fn mc_data() -> (Vec<Vec<Vec<PathBuf>>>) {

    let MC_multi_player: Vec<PathBuf> = data_dir_search(PathBuf::from(".minecraft\\journeymap\\data\\mp"));
    let MC_single_player: Vec<PathBuf> = data_dir_search(PathBuf::from(".minecraft\\journeymap\\data\\sp"));

    let Prism_multi_player: Vec<PathBuf> = data_instance_search(PathBuf::from("PrismLauncher\\instances"), PathBuf::from("mp"));
    let Prism_single_player: Vec<PathBuf> = data_instance_search(PathBuf::from("PrismLauncher\\instances"), PathBuf::from("sp"));

    return [[MC_multi_player, MC_single_player].to_vec(), [Prism_multi_player, Prism_single_player].to_vec()].to_vec()
}


fn data_dir_search(path_to_dir: PathBuf) -> Vec<PathBuf> {

    let mut paths_found: Vec<PathBuf> = Vec::new();

    if let Some(data_dir) = BaseDirs::new() {

        let full_path = &data_dir.data_dir().join(path_to_dir);

        if let Ok(paths) = fs::read_dir(full_path){

            for path in paths {
                
                paths_found.push(path.unwrap().path().clone());

            }

        } else {

            println!("MC not installed!")

        }
    }

    return paths_found

}

fn data_instance_search(path_to_dir: PathBuf, world_type: PathBuf) -> Vec<PathBuf> {

    let mut paths_found: Vec<PathBuf> = Vec::new();

    if let Some(data_dir) = BaseDirs::new() {

        let full_path = &data_dir.data_dir().join(path_to_dir);


        
        if let Ok(paths) = fs::read_dir(full_path){
            for path in paths {
            
                if path.as_ref().unwrap().path().is_dir() == true {

                    let check_journey: PathBuf = [path.as_ref().unwrap().path(), PathBuf::from(".minecraft\\journeymap\\data"), world_type.clone()].iter().collect();

                    if check_journey.exists(){
                        let instance_paths = fs::read_dir(check_journey).unwrap();

                        for world in instance_paths {
                            paths_found.push(world.unwrap().path().clone());
                        }
                    }


                }
            }
        } else {
            println!("Prism not installed!")
        }

    }

    return paths_found;

}



pub fn get_last_world() -> PathBuf{

    let file_path = Path::new("worldSave.txt");

    if let Ok(path_string) = fs::read_to_string(file_path){
        if let Ok(path_to_world) = paths_as_strings::decode_path(&path_string){
            return path_to_world;
        } 
    } else {
        println!("file does not exist")
    }
    
    return PathBuf::new();

    
}

