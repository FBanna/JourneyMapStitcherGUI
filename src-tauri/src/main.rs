// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_world;
mod get_waypoint;

use std::path::Path;
use std::path::PathBuf;
use std::thread::JoinHandle;

use axum::extract::path;
use futures_util::lock;
use get_waypoint::WayPoint;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, EncodableLayout, Rgba, DynamicImage};
use tracing_subscriber::fmt::format;
use std::env;
use turbojpeg;
use std::fs::write;

use image::imageops::FilterType;
use std::fmt;
use image::ImageFormat;

use axum::{
    routing::{get, post},
    response::{AppendHeaders, IntoResponse},
    Json, Router,
    extract::State,
    extract::Path as axumPath,
    http::{header, StatusCode}, body::StreamBody,

};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use futures_util::stream::{self, Stream};

use tokio;

use std::collections::HashMap;
use tokio::task;
use std::thread;
use tokio_util;
use std::io::{BufWriter, Cursor};
use std::sync::{Arc, Mutex};
use std::time::Instant;


use bytes::Bytes;

use http_body::Full;

use std::fs;

use tauri::State as TauriState;
use tauri::Manager;
use tauri::Window;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::task::JoinSet;



/*

https://betterprogramming.pub/front-end-back-end-communication-in-tauri-implementing-progress-bars-and-interrupt-buttons-2a4efd967059


use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: f64,
}

#[tauri::command]
pub async fn do_some_long_task(window: Window){
  let mut pct = 0.0;
  loop {
      // computations ...
      window.emit("PROGRESS", Payload { pct }).unwrap();
  }
}
 */




#[tauri::command]
async fn stitch(x1: f32, y1: f32, x2: f32, y2: f32, mut radius: f32, style: String, dimension: String, tauriCurrentPath: TauriState<'_, CurrentPath>) -> Result<(), ()>{
    println!("stitch called with {} {} {} {}", x1, y1, x2, y2);

    let mut startxtile: i32 = 0;
    let mut startytile: i32 = 0;

    let mut xsize: f32 = 0.0;
    let mut ysize: f32 = 0.0;

    let mut x: i32;
    let mut y: i32;

    let mut save: String;
    let mut targetfile: String;


    if style == "span"{
        xsize = ((x1-x2)/512.0).round().abs();
        ysize = ((y1-y2)/512.0).round().abs();
        

        if x1 < x2{
            startxtile = ((x1/512.0).round()) as i32;
        } else {
            startxtile = ((x2/512.0).round()) as i32;
        }

        if y1 < y2{
            startytile = ((y1/512.0).round()) as i32;
        } else {
            startytile = ((y2/512.0).round()) as i32;
        }

        println!("image going from {}, {} to {}, {} starting at {}, {} with a size of {}, {}", x1,y1,x2,y2,startxtile,startytile,xsize,ysize);
    } else if style == "center" {
        if radius == 0.0 {
            radius = 32512.0;
        }
        
        xsize = ((radius*2.0)/512.0).round();
        ysize = ((radius*2.0)/512.0).round();

        startxtile = ((((x1)-(radius))/512.0).round()) as i32;

        startytile = ((((y1)-(radius))/512.0).round()) as i32;

        println!("image with center at {}, {} and radius of {} starting at {}, {}", x1, y1, radius, startxtile, startytile);
        println!("({}, {})\nto\n({}, {})\n", ((x1 as i32)-(radius as i32)), ((y1 as i32)-(radius as i32)), ((x1 as i32)+(radius as i32)), ((y1 as i32)+ (radius as i32)));
    
    } else {
        println!("enter valid style!");
    }

    println!("starting at: ({}, {})\n", startxtile, startytile);
    println!("x width = {} tiles\n", xsize);
    println!("y width = {} tiles\n", ysize);

    /////////////////
    //             //
    //  SPLITTING  //
    //             //
    /////////////////

    let mut imagesizex: f32 = xsize;
    let mut imagesizey: f32 = ysize;

    if xsize > 127.0 || ysize > 127.0 {

        while imagesizex > 127.0 {
            println!("x size = {} too large SPLITTING!", imagesizex);
            imagesizex = imagesizex/2.0;
        }

        

        while imagesizey > 127.0 {
            println!("y size = {} too large SPLITTING!", imagesizey);
            imagesizey = imagesizey/2.0;
        }


        println!("x size now = {}", imagesizex);
        println!("y size now = {}", imagesizey);
    }

    
    let neededx: i32 = ((xsize/imagesizex).ceil()) as i32;
    let neededy: i32 = ((ysize/imagesizey).ceil()) as i32;


    let imagesizex: i32 = imagesizex.round() as i32;
    let imagesizey: i32 = imagesizey.round() as i32;
    //println!("{} {} {} {}", neededx, neededy, imagesizex, imagesizey);
    //println!("{} {}", xsize, ysize);
    //println!("creating {} image/s,", neededx * neededy);

    //println!("im HERE {} {}", neededx, neededy);

    //let mut tasks = FuturesUnordered::new();
    //let mut set = JoinSet::new();



    let locked_path = tauriCurrentPath.path.lock().expect("POISONED");

    let mut world_path = locked_path.clone();

    world_path.push(dimension);

    println!("{:?}", locked_path);
    drop(locked_path);
    
    

    for ximages in 0 .. neededx {

        for yimages in 0 .. neededy {
            
            x = (ximages*imagesizex) + startxtile;
            y = (yimages*imagesizey) + startytile;

            save = format!("out {},{}.jpg",ximages,yimages);
            //println!("{}, {}, {}, {}, {}", x, y, imagesizex, imagesizey, save);

            //tasks.push(task::spawn(async move {
            //let dimension = dimension.clone();

            let world_path = world_path.clone();
            //set.spawn(async move {
            //    creation(x,y,imagesizex,imagesizey,save, world_path).await
            //});

            tokio::spawn(async move{
                creation(x,y,imagesizex,imagesizey,save, world_path).await
            });

            
            //}));

        }
    }

    //while let Some(res) = set.join_next().await {
    //    let out = res?;
        // ...
    //}
    Ok(())

    


}

async fn creation(startingx: i32, startingy: i32, width: i32, height: i32, out: String, world_path: PathBuf){
    // make image with width and height

    let mut x: i32;
    let mut y: i32;
    let mut targetfile: PathBuf;

    let mut imgbuf = image::ImageBuffer::new((width*512) as u32,(height*512) as u32);
    println!("{} {} {} {} {}", startingx*512, startingy*512, width*512, height*512, out);

    for xaxis in 0 .. width {

        x = xaxis + startingx;

        for yaxis in 0 .. height {

            
            y = yaxis + startingy;
            

            //targetfile = format!("{}/day/{},{}.png", dimension, x, y);
            targetfile = world_path.clone();
            targetfile.push(format!("day/{},{}.png", x, y));

            //let path = Path::new(&targetfile);

            //if path.exists() {
                //println!("{} exists!", targetfile);
            if let Ok(tempimg) = image::open(targetfile) {
        
                imgbuf.copy_from(&tempimg, (512*xaxis) as u32, (512*yaxis) as u32);
            }
                //let tempimg = image::open(targetfile).unwrap();
                
                //imgbuf.copy_from(&tempimg, (xaxis*512) as u32, (yaxis*512) as u32);

                //let tile_scaled = tempimg.resize(256, 256, FilterType::Nearest);
                //let save: String = format!("./tiles/{}/{}.png", x, y);

                //fs::create_dir(format!("./tiles/{}",x));

                
                //tile_scaled.save(save).unwrap();

            //}
        }
    }


    //imgbuf.save("test.png").unwrap();
    //image::save_buffer("image.png", imgbuf, width as u32, height as u32, image::ColorType::Rgb8).unwrap()
    //image::save_buffer(&Path::new("image.jpg"), imgbuf, 800, 600, image::RGBA(8))


    
    // compress `image` into JPEG with quality 95 and 2x2 chrominance subsampling
    // https://docs.rs/turbojpeg/0.5.4/turbojpeg/enum.Subsamp.html
    let jpeg_data = turbojpeg::compress_image(&imgbuf, 100, turbojpeg::Subsamp::Sub2x2).unwrap();

    // write the JPEG to disk
    std::fs::write(out, &jpeg_data);
}



#[tokio::main]
async fn server(stateholder: Arc<Mutex<PathBuf>>) {
    

    let state = AppState {stateholder};
    tracing_subscriber::fmt::init();
    
    
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`

        .route("/:reload/:dim/:zoom/:x/:y", get(root))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root(State(state): State<AppState>, axumPath((reload,dim , z, x,y)): axumPath<(i32, String, i32, i32, i32)>) ->  Result<impl axum::response::IntoResponse, ()> {
    
    //let start_time = Instant::now();

    let mut found: bool = false;

    

    
    //let mut locked_path = state.path.lock().expect("mutex was poisoned");
    //println!("{}", locked_path.file_name().unwrap().to_str().unwrap().to_string());
    //drop(locked_path);
    
    let locked_path = state.stateholder.lock().expect("POISONED");
    
    //*counter = *counter + 1;

    //println!("{}", locked_path.file_name().unwrap().to_str().unwrap().to_string());
    let path_to_world = &locked_path.clone();
    //println!("{}", path_to_world.to_str().unwrap().to_string());
    //let checkfile: String = format!("{}/{}/day/{},{}.png",locked_path.file_name().unwrap().to_str().unwrap().to_string(),dim,x,y);
    drop(locked_path);
    //println!("{}|{}", z, checkfile);
    

    //let path = p::new(&checkfile);

    //if path.exists() {

    let mut currentx: i32;
    let mut currenty: i32;

    

    let mut targetfile: PathBuf;

    let maxzoom: i32 = 6;
    let imagedesity: i32 = 2_i32.pow((maxzoom-z) as u32);
    let multiplyer: u32 = (256.0/imagedesity as f32).floor() as u32;
 
    //let onetilesize: i32 = (256.0/imagedesity as f32).floor() as i32;

    let startingx: i32 = x * imagedesity;
    let startingy: i32 = y * imagedesity;

    let mut scaled_image: DynamicImage;

    //let elapsed_copying;



    //let mut imgbuf = image::ImageBuffer::new((512 * imagedesity) as u32,(512 * imagedesity) as u32);
    //OR
    let mut imgbuf = image::ImageBuffer::new(256, 256);




    for xaxis in 0 .. imagedesity {

        currentx = xaxis + startingx;

        for yaxis in 0 .. imagedesity {

            currenty = yaxis + startingy;

            match dim.as_ref() {

                "the_nether" => {
                    for layer in (0..=7).rev(){
                        targetfile = path_to_world.join(format!(r".\the_nether\{}\{},{}.png", layer,currentx, currenty));

                        if let Ok(tempimg) = image::open(targetfile) {
                            //let mutex_elapsed = mutex_time.elapsed();
            
                            //println!("check: {:?}",mutex_elapsed);
            
                            found = true;
                            scaled_image = tempimg.resize(multiplyer,multiplyer, FilterType::Nearest);
            
                            imgbuf.copy_from(&scaled_image,multiplyer*xaxis as u32, multiplyer*yaxis as u32);
                            break;

                        }
                    }
                },


                _ => {
                    
                    targetfile = path_to_world.join(format!(r".\{}\day\{},{}.png", dim,currentx, currenty));

                    //OR
                    //let mutex_time = Instant::now();
                    if let Ok(tempimg) = image::open(targetfile) {
                        //let mutex_elapsed = mutex_time.elapsed();

                        
                        //println!("check: {:?}",mutex_elapsed);

                        found = true;
                        scaled_image = tempimg.resize(multiplyer,multiplyer, FilterType::Nearest);

                        imgbuf.copy_from(&scaled_image,multiplyer*xaxis as u32, multiplyer*yaxis as u32);

                    }
                }
            }

            //targetfile = [path_to_world, &PathBuf::from(format!(r".\{}\day\{},{}.png", dim,currentx, currenty))].iter().collect();
        }
    }
    //let elapsed_time = start_time.elapsed();
    

    

    //let body;
    //if found == true {
    match found {
        true => {
            let headers = AppendHeaders([
                (header::CONTENT_TYPE, "image/png"),
                (header::CONTENT_DISPOSITION, "inline; filename=\"test.png\"")
            ]);
    
            //let start_time_1 = Instant::now();
            
            /*
            let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    
            imgbuf.write_to(&mut buffer, ImageFormat::Png).unwrap();
    
            let raw: Vec<u8> = buffer.into_inner().unwrap().into_inner();*/
    
            let mut raw = Vec::new();
            imgbuf.write_to(&mut Cursor::new(&mut raw), ImageFormat::Png).unwrap();
            let bytes = Bytes::from(raw);
    
            // Convert stream to axum HTTP body
            //let bytes = Bytes::from(raw);
            let body = Full::new(bytes);
            //let elapsed_time_1 = start_time_1.elapsed();
    
            //println!("Elapsed time: {:?}, {:?}, {:?}", elapsed_time,elapsed_time_1,found);
            Ok((headers, body))
        },
        false => Err(())
    }
        // Headers
}

#[tauri::command]
async fn select_world_window(app: tauri::AppHandle) {

    let _ = tauri::WindowBuilder::new(&app, "select_world", tauri::WindowUrl::App("select_world/index.html".into()))
        .title("Select your world")
        .inner_size(1023.0, 456.0)
        .build();
}


//gets all current MC worlds and gives to frontend
#[tauri::command]
fn get_world(tauriNewPath: TauriState<NewPath>) -> (Vec<Vec<Vec<PathBuf>>>){

    let paths = get_world::mc_data();

    let mut locked_paths = tauriNewPath.paths.lock().expect("POISONED");

    *locked_paths = paths.clone();
    drop(locked_paths);

    
    return paths;

}

//sets current world by changing mutex and updating frontend
#[tauri::command]
fn set_world(launcher:usize, list: usize, index: usize, tauriCurrentPath: TauriState<CurrentPath>, tauriNewPath: TauriState<NewPath>) {

    let mut locked_path = tauriCurrentPath.path.lock().expect("POISONED");

    let selected_paths = tauriNewPath.paths.lock().expect("POISONED");

    //*locked_path = selected_paths[list][index].clone();
    *locked_path = selected_paths[launcher][list][index].clone();

}

//saves last world to disc
#[tauri::command]
fn store_last_world(tauriPath: TauriState<CurrentPath>) {

    let file_path = Path::new("worldSave.txt");

    let mut locked_path = tauriPath.path.lock().expect("POISONED");


    let encoded = paths_as_strings::encode_path(&*locked_path);
    fs::write(file_path, encoded.as_bytes()).expect("Unable to write file");

    drop(locked_path);
}

#[tauri::command]
fn get_selected(tauri_new_paths: TauriState<NewPath>, tauri_current_path: TauriState<CurrentPath>) -> Vec<usize> {
    
    let current_path = tauri_current_path.path.lock().expect("POISONED");

    let new_paths = tauri_new_paths.paths.lock().expect("POISONED");


    for launcher in 0..2 {
        for list in 0..new_paths[launcher].len() {
            for item in 0..new_paths[launcher][list].len() {

                if new_paths[launcher][list][item] == current_path.clone() {

                    return [launcher, list, item].to_vec()
                }

            }
        }
    }

    return [0, 0, 0].to_vec()

}   


#[tauri::command]
fn get_waypoints(tauri_current_path: TauriState<CurrentPath> ) -> Vec<WayPoint> { // -> Vec<String, Vec<i32>, Vec<i32>>
    
    let locked_path = tauri_current_path.path.lock().expect("POISONED");

    let waypoints = get_waypoint::waypoint_paths(locked_path.clone());
    drop(locked_path);

    return waypoints;
}


struct CurrentPath {
    path: Arc<Mutex<PathBuf>>
}

struct NewPath {
    //paths: Arc<Mutex<Vec<Vec<PathBuf>>>>

    paths: Arc<Mutex<Vec<Vec<Vec<PathBuf>>>>>
}

#[derive(Clone)]
struct AppState {
    stateholder: Arc<Mutex<PathBuf>>
}




#[tokio::main]
async fn main() {
    

    let temppath = get_world::get_last_world();

    let path_original: Arc<Mutex<PathBuf>> = Arc::new(Mutex::new(temppath));
    let pathServer = path_original.clone();
    let pathChange = path_original.clone();

    thread::spawn(move || {
        println!("server starting"); 
        server(pathServer);  
    });

    tauri::Builder::default()
        .manage(CurrentPath { path: pathChange })
        .manage(NewPath {paths: Arc::new(Mutex::new(Vec::new()))})
        .invoke_handler(tauri::generate_handler![stitch, select_world_window, get_world, set_world, store_last_world, get_selected, get_waypoints])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
