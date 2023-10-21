// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive};
use std::env;

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

fn creation(startingx: i32, startingy: i32, width: i32, height: i32, out: String){
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
            let path = Path::new(&targetfile);

            if path.exists() {
                println!("{} exists!", targetfile);

                let tempimg = image::open(targetfile).unwrap();
                imgbuf.copy_from(&tempimg, (xaxis*512) as u32, (yaxis*512) as u32);

            }
        }
    }


    imgbuf.save("test.jpg").unwrap();
    //image::save_buffer("image.png", imgbuf, width as u32, height as u32, image::ColorType::Rgb8).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stitch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
