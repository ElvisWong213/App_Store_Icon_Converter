mod image_process;

use std::io::stdin;

use image_process::ImageProcess;

fn main() {
    let mut input_path: String = String::new();
    let mut output_path: String = String::new();
    println!("Input path");
    stdin().read_line(&mut input_path).unwrap();
    println!("Output path");
    stdin().read_line(&mut output_path).unwrap();
    let mut img_p = ImageProcess::new(input_path, output_path);
    img_p.start();
}

