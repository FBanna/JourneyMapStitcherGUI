use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use tracing_subscriber::field::debug;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use std::result;
use std::{fs};
use directories::{BaseDirs, UserDirs, ProjectDirs};
use tauri::api::path::data_dir;
use std::io::prelude::*;
use std::borrow::Cow;
use paths_as_strings;

#[derive(Deserialize)]
struct RawWayPoint{
    id: String,
    name: String,
    icon: String,
    colorizedIcon: String,
    x: i32,
    y: i32,
    z: i32,
    r: i32,
    g: i32,
    b: i32,
    enable: bool,
    r#type: String,
    origin: String,
    dimensions: Vec<String>,
    persistent: bool,
    showDeviation: bool,
    iconColor: i32,
    customIconColor: bool
}

#[derive(Serialize)]
pub struct Colour{
    r:i32,
    g:i32,
    b:i32
}

#[derive(Serialize)]
pub struct WayPoint{
    name: String,
    x: i32,
    z: i32,
    colour: Colour,
    dimensions: Vec<String>
}

trait translation{
    fn translate(&self) -> WayPoint;
}
impl translation for RawWayPoint{
    fn translate(&self) -> WayPoint{
        let new = WayPoint{
            name: self.name.clone(),
            x: self.x,
            z: self.z,
            colour: Colour{
                r: self.r,
                g: self.g,
                b: self.b
            },
            dimensions: self.dimensions.clone()
        };
        return new;
    }
}


pub fn waypoint_paths(path_to_world:PathBuf) -> Vec<WayPoint> {
    let mut waypoints: Vec<WayPoint> = Vec::new();
    let mut new_way_point: WayPoint;

    let path_to_waypoints = &path_to_world.join("waypoints/");

    if let Ok(paths) = fs::read_dir(path_to_waypoints) {
        for path in paths {
            //println!("{:?}", path);
            let file = File::open(path.unwrap().path().clone()).expect("msg");
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let json: RawWayPoint = serde_json::from_reader(reader).expect("msg");

            new_way_point = json.translate();

            waypoints.push(new_way_point);

            


        }
    }
    //println!("{:?}", waypoints)
    return waypoints
}