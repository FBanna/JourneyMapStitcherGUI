// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stitch(x1: f32, y1: f32, x2:f32, y2:f32, radius: f32, style: String){
    println!("stitch called with {} {} {} {}", x1, y1, x2, y2);

    let mut startxtile: f32 = 0.0;
    let mut startytile: f32 = 0.0;

    let mut xsize: f32 = 0.0;
    let mut ysize: f32 = 0.0;

    let mut imagesizex: f32 = 0.0;
    let mut imagesizey: f32 = 0.0;

    let mut x: i32;
    let mut y: i32;

    let save: String;

    if style == "span"{
        xsize = f32::abs((x1-x2)/512.0);
        ysize = f32::abs((y1-y2)/512.0);

        


        if x1 < x2{
            startxtile = f32::abs(x1/512.0);
        } else {
            startxtile = f32::abs(x2/512.0);
        }

        if y1 < y2{
            startytile = f32::abs(y1/512.0);
        } else {
            startytile = f32::abs(y2/512.0);
        }

        println!("image going from {}, {} to {}, {} starting at {}, {} with a size of {}, {}", x1,y1,x2,y2,startxtile,startytile,xsize,ysize);
    } else if style == "center" {
        if radius == 0.0 {
            let radius = 32512;
        }

        xsize = f32::abs((radius*2.0)/512.0);
        ysize = f32::abs((radius*2.0)/512.0);

        startxtile = f32::abs((x1-radius)/512.0);
        startytile = f32::abs((y1-radius)/512.0);

        println!("image with center at {}, {} and radius of {} starting at {}, {}", x1, y1, radius, startxtile, startytile);
    } else {
        println!("enter valid style!");
    }

    /////////////////
    //             //
    //  SPLITTING  //
    //             //
    /////////////////

    imagesizex = xsize;
    imagesizex = xsize;

    if xsize > 127.0 || ysize > 127.0 {

        while imagesizex > 127.0 {
            println!("x size = {} too large SPLITTING!", imagesizex);
            imagesizex = f32::abs(imagesizex/2.0);
        }

        

        while imagesizey > 127.0 {
            println!("y size = {} too large SPLITTING!", imagesizey);
            imagesizey = f32::abs(imagesizey/2.0);
        }


        let imagesizex: i32 = imagesizex.round() as i32;
        let imagesizey: i32 = imagesizey.round() as i32;


        println!("x size now = {}", imagesizex);
        println!("y size now = {}", imagesizey);
    }

    let neededx: i32 = (xsize/imagesizex).ceil() as i32;
    let neededy: i32 = (ysize/imagesizey).ceil() as i32;

    println!("creating {} image/s,", neededx * neededy);

    for ximages in 0 .. neededx {
        for yimages in 0 .. neededy {
            x = (ximages*imagesizex) + startxtile;
            y = (yimages*imagesizey) + startytile;

            save = format!("out {},{}",ximages,yimages);
        }
    }


}

fn creation(startingx: f32, startingy: f32, width: f32, height: f32, out: String){
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![stitch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
