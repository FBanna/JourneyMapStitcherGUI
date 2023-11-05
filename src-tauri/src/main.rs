// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path as p, fs};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, EncodableLayout, Rgba};
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
    extract::Path,
    http::{header, StatusCode}, body::StreamBody,

};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use futures_util::stream::{self, Stream};
use std::sync::Arc;

use tokio;

use std::collections::HashMap;
use tokio::task;
use std::thread;
use tokio_util;
use std::io::{BufWriter, Cursor};

use bytes::Bytes;

use http_body::Full;

#[tauri::command]
fn stitch(x1: f32, y1: f32, x2:f32, y2:f32, mut radius: f32, style: String){
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
        xsize = ((x1-x2)/512.0).round();
        ysize = ((y1-y2)/512.0).round();
        

        


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

    for ximages in 0 .. neededx {
        for yimages in 0 .. neededy {
            x = (ximages*imagesizex) + startxtile;
            y = (yimages*imagesizey) + startytile;

            save = format!("out {},{}",ximages,yimages);
            //println!("{}, {}, {}, {}, {}", x, y, imagesizex, imagesizey, save);
            creation(x,y,imagesizex,imagesizey,save);
            
        }
    }


}

#[tauri::command]
fn get_tile(x: i32, y: i32) {
    println!("get_tile called with {} {}", x, y);

    let targetfile: String = format!("day/{},{}.png", x, y);

    let path = p::new(&targetfile);

    if path.exists() {
        println!("found tile {},{}.png", x, y);
        let tile = image::open(targetfile).unwrap();
        let tile_scaled = tile.resize(256, 256, FilterType::Nearest);
        let save: String = format!("tiles/{},{}.png", x, y);
        tile_scaled.save(save).unwrap();

    }
}

async fn creation(startingx: i32, startingy: i32, width: i32, height: i32, out: String){
    // make image with width and height

    let mut x: i32;
    let mut y: i32;
    let mut targetfile: String;

    let mut imgbuf = image::ImageBuffer::new((width*512) as u32,(height*512) as u32);
    //println!("{} {} {} {} {}", startingx*512, startingy*512, width*512, height*512, out);
    for xaxis in 0 .. width {
        for yaxis in 0 .. height {

            x = xaxis + startingx;
            y = yaxis + startingy;
            

            targetfile = format!("day/{},{}.png", x, y);
            let path = p::new(&targetfile);

            if path.exists() {
                println!("{} exists!", targetfile);

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

#[tokio::main]
async fn server() {
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/:z/:x/:y", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root(Path((z, x,y)): Path<(i32, i32, i32)>) ->  impl axum::response::IntoResponse {


    let targetfile: String = format!("day/{},{}.png",x,y);
    println!("{}",targetfile);

    let path = p::new(&targetfile);

    if path.exists() {
        let tempimg = image::open(&targetfile).unwrap();

        let tile_scaled = tempimg.resize(256, 256, FilterType::Nearest);

        //let save: String = format!("./tiles/{}/{}.png", x, y);

        //fs::create_dir(format!("./tiles/{}",x));

        
        //tile_scaled.save(save).unwrap();


        //let mut buffer = BufWriter::new(Cursor::new(Vec::new()));

        //tile_scaled.write_to(&mut buffer, ImageFormat::Png).unwrap();
        //let bytes = tile_scaled.as_bytes();
        let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
        tile_scaled.write_to(&mut buffer, ImageFormat::Png).unwrap();
        let raw: Vec<u8> = buffer.into_inner().unwrap().into_inner();
        

        //let stream = ReaderStream::new(&*bytes);

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

    } else { //if no file exists

        let mut imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(256 as u32, 256 as u32);

        let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
        imgbuf.write_to(&mut buffer, ImageFormat::Png).unwrap();
        let raw: Vec<u8> = buffer.into_inner().unwrap().into_inner();
        

        //let stream = ReaderStream::new(&*bytes);

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

    





    println!("file created");

}


#[tokio::main]
async fn main() {

    thread::spawn(move || {
        server();
        println!("server starting");
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stitch, get_tile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}
