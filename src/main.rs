use enigo::{Enigo, MouseControllable};
use env::current_dir;
use image::{GenericImageView, Pixel, Rgb};
use screenshot_rs::screenshot_full;
use std::{env, error::Error, fs, thread::sleep, time::Duration};

fn main() {
    let enigo = Enigo::new();
    let path = current_dir().unwrap().to_str().unwrap().to_string() + "/tempscreenshot.png";
    match remove_screenshot(&path) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    };
    loop {
        let (x, y) = enigo.mouse_location();
        match get_rgb(&path, x, y) {
            Ok(_) => (),
            Err(e) => {
                println!("{e}");
                break;
            }
        };
        sleep(Duration::from_millis(5));
    }
}

fn remove_screenshot(path: &String) -> Result<(), Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(())
}

fn get_rgb(path: &String, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
    screenshot_full(path.clone());
    let image = image::open(path).expect("Failed to open image");
    let pixel = image.get_pixel(x as u32, y as u32);

    // Handle both Rgb<u8> and Rgba<u8>
    match pixel.channels() {
        [r, g, b] => {
            print_colored_text(x, y, *r, *g, *b);
        }
        [r, g, b, _a] => {
            print_colored_text(x, y, *r, *g, *b);
        }
        _ => {
            panic!("System uses an unknown pixel format.")
        }
    }

    remove_screenshot(path)
}

fn print_colored_text(x: i32, y: i32, r: u8, g: u8, b: u8) {
    let rgb_pixel: Rgb<u8> = Rgb([r.clone(), g.clone(), b.clone()]);
    let formated_pixel = format!("({}, {}): {:?}", x, y, rgb_pixel);
    let color_code = format!("\x1B[38;2;{};{};{}m", r, g, b);
    let reset_code = "\x1B[0m";
    println!("{} -> {} ■■■■■ {}", formated_pixel, color_code, reset_code);
}
