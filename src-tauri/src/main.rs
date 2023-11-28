// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_world;

use std::path::Path;
use std::path::PathBuf;

use axum::extract::path;
use futures_util::lock;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, EncodableLayout, Rgba, DynamicImage};
use std::env;
use turbojpeg;
use std::fs::write;
use tauri::State as TauriState;
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

use bytes::Bytes;

use http_body::Full;




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
async fn stitch(x1: f32, y1: f32, x2:f32, y2:f32, mut radius: f32, style: String, dimension: String){
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
    for ximages in 0 .. neededx {

        

        for yimages in 0 .. neededy {
            
            x = (ximages*imagesizex) + startxtile;
            y = (yimages*imagesizey) + startytile;

            save = format!("out {},{}.jpg",ximages,yimages);
            //println!("{}, {}, {}, {}, {}", x, y, imagesizex, imagesizey, save);

            creation(x,y,imagesizex,imagesizey,save, dimension.clone()).await;

        }
    }


}

async fn creation(startingx: i32, startingy: i32, width: i32, height: i32, out: String, dimension: String){
    // make image with width and height

    let mut x: i32;
    let mut y: i32;
    let mut targetfile: String;

    let mut imgbuf = image::ImageBuffer::new((width*512) as u32,(height*512) as u32);
    println!("{} {} {} {} {}", startingx*512, startingy*512, width*512, height*512, out);
    for xaxis in 0 .. width {

        x = xaxis + startingx;

        for yaxis in 0 .. height {

            
            y = yaxis + startingy;
            

            targetfile = format!("{}/day/{},{}.png", dimension, x, y);
            let path = Path::new(&targetfile);

            if path.exists() {
                //println!("{} exists!", targetfile);

                let tempimg = image::open(targetfile).unwrap();
                
                imgbuf.copy_from(&tempimg, (xaxis*512) as u32, (yaxis*512) as u32);

                //let tile_scaled = tempimg.resize(256, 256, FilterType::Nearest);
                //let save: String = format!("./tiles/{}/{}.png", x, y);

                //fs::create_dir(format!("./tiles/{}",x));

                
                //tile_scaled.save(save).unwrap();

            }
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

#[derive(Clone)]
struct AppState {path: Arc<Mutex<PathBuf>>}

#[tokio::main]
async fn server(path: Arc<Mutex<PathBuf>>) {
    

    let state = AppState {path};
    tracing_subscriber::fmt::init();
    
    
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/:p/:z/:x/:y", get(root))
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


async fn root(State(state): State<AppState>, axumPath((dim , z, x,y)): axumPath<(String, i32, i32, i32)>) ->  impl axum::response::IntoResponse {
    
    let mut locked_path = state.path.lock().expect("mutex was poisoned");
    println!("{}", locked_path.file_name().unwrap().to_str().unwrap().to_string());
    drop(locked_path);

    let checkfile: String = format!("/{}/day/{},{}.png",dim,x,y);

    println!("{}|{}", z, checkfile);
    

    //let path = p::new(&checkfile);

    //if path.exists() {

    let mut currentx: i32;
    let mut currenty: i32;

    

    let mut targetfile: String;

    let maxzoom: i32 = 6;
    let imagedesity: i32 = 2_i32.pow((maxzoom-z) as u32);
 
    //let onetilesize: i32 = (256.0/imagedesity as f32).floor() as i32;

    let startingx: i32 = x * imagedesity;
    let startingy: i32 = y * imagedesity;



    let mut imgbuf = image::ImageBuffer::new((512 * imagedesity) as u32,(512 * imagedesity) as u32);
    




    for xaxis in 0 .. imagedesity {

        currentx = xaxis + startingx;

        for yaxis in 0 .. imagedesity {

            currenty = yaxis + startingy;
            

            targetfile = format!("{}/day/{},{}.png", dim, currentx, currenty);
            //println!("{}", targetfile);
            let path = Path::new(&targetfile);

            if path.exists() {
                //println!("{} exists!", targetfile);

                let tempimg = image::open(targetfile).unwrap();
                
                
                //println!("{}", onetilesize);
                imgbuf.copy_from(&tempimg, (512*xaxis) as u32, (512*yaxis) as u32);


                
                
                //let save: String = format!("./tiles/{}/{}.png", x, y);

                //fs::create_dir(format!("./tiles/{}",x));

                
                //imgbuf.save(save).unwrap();

            }
        }
    }
    
    //let tempimg = image::open(&checkfile).unwrap();

    //let tile_scaled = tempimg.resize(256, 256, FilterType::Nearest);
    let tile_scaled = &DynamicImage::ImageRgba8(imgbuf).resize(256, 256, FilterType::Nearest);

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));

    tile_scaled.write_to(&mut buffer, ImageFormat::Png).unwrap();

    let raw: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    // Convert stream to axum HTTP body
    let bytes = Bytes::from(raw);
    let body = Full::new(bytes);

    // Headers
    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "image/png"),
        (header::CONTENT_DISPOSITION, "inline; filename=\"test.png\"")
    ]);
    
    // Return
    
    return(headers, body);

}

#[tauri::command]
async fn select_world_window(app: tauri::AppHandle) {

    let _ = tauri::WindowBuilder::new(&app, "select_world", tauri::WindowUrl::App("select_world/index.html".into()))
        .title("Select your world")
        .inner_size(1023.0, 456.0)
        .build();
}
pub struct Storage {
    store: Mutex<i32>
}
#[tauri::command]
fn get_world(storage: TauriState<Storage>) -> (Vec<PathBuf>, Vec<PathBuf>){
    //let mut MC_multi_player: Vec<String> = Vec::new();
    //let mut MC_single_player: Vec<String> = Vec::new();

    
    let mut counter = storage.store.lock().unwrap();
    *counter = *counter + 1;

    println!("{}", *counter);
    drop(counter);

    let (MC_multi_player_path, MC_single_player_path ) = get_world::mc_data();
    /*
    for item in &MC_multi_player_path {
        MC_multi_player.push(item.file_name().unwrap().to_str().unwrap().to_string());
        println!("mp item: {}", item.file_name().unwrap().to_str().unwrap().to_string());
    }

    for item in &MC_single_player_path {
        MC_single_player.push(item.file_name().unwrap().to_str().unwrap().to_string());
        println!("mp item: {}", item.file_name().unwrap().to_str().unwrap().to_string());
    }
    */

    //return (MC_multi_player, MC_single_player);
    return (MC_multi_player_path, MC_single_player_path);

}




#[tokio::main]
async fn main() {
    let mut temppath = PathBuf::new();
    temppath.push("hello\\bye");
    let path_original: Arc<Mutex<PathBuf>> = Arc::new(Mutex::new(temppath));

    let path = path_original.clone();

    thread::spawn(move || {
        println!("server starting");
        server(path);  
    });

    tauri::Builder::default()
        .manage(Storage { store: Mutex::new(0) })
        .invoke_handler(tauri::generate_handler![stitch, select_world_window, get_world])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}
