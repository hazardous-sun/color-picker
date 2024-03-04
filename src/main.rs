use enigo::Enigo;
use enigo::MouseControllable;
use env::current_dir;
use image::{GenericImageView, Pixel, Rgb};
use screenshot_rs::screenshot_full;
use std::env;
use std::fs;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn get_rgb(path: &String, x: i32, y: i32) {
    screenshot_full(path.clone());
    let image = image::open(path).expect("Failed to open image");
    let pixel = image.get_pixel(x as u32, y as u32);

    // Handle both Rgb<u8> and Rgba<u8>
    match pixel.channels() {
        [r, g, b] => {
            let rgb_pixel: Rgb<u8> = Rgb([r.clone(), g.clone(), b.clone()]);
            println!("({}, {}): {:?}", x, y, rgb_pixel);
        }
        [r, g, b, _a] => {
            let rgb_pixel: Rgb<u8> = Rgb([r.clone(), g.clone(), b.clone()]);
            println!("({}, {}): {:?}", x, y, rgb_pixel);
        }
        _ => {
            eprintln!("Unexpected pixel format");
            exit(2);
        }
    }

    match fs::remove_file(path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error removing file: {e}");
            exit(1);
        }
    }
}

fn color_picker() {
    let enigo = Enigo::new();
    let path = current_dir().unwrap().to_str().unwrap().to_string() + "/tempscreenshot.png";
    while true {
        let (x, y) = enigo.mouse_location();
        get_rgb(&path, x, y);
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    color_picker();
}
