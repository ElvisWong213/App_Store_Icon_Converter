use core::panic;
use std::{fs::create_dir_all, path::Path, process::exit};
use image::{imageops, open, GenericImage, ImageBuffer, Rgba};

use crate::output_format::OutputFormat;

#[derive(Debug)]
pub struct ImageProcess {
    input_path: String,
    output_path: String,
}

impl ImageProcess {
    pub fn new(input_path: String, output_path: String) -> Self {
        let input = input_path.replace("\n", "");
        let mut output = output_path.replace("\n", "");
        if input.is_empty() || output.is_empty() {
            panic!("path cannot be empty")
        }
        output = output + "/output";
        Self { 
            input_path: input,
            output_path: output,
        }
    } 

    pub fn run(&mut self) {
        let image = self.check_input_image().unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(0);
        });

        let rounded_corners_image = self.rounded_corners(&image, 234);
        let app_store_image = self.insert_transparent_border(&rounded_corners_image);
        let app_store_outputs = OutputFormat::app_store_outputs();

        // Create folder
        self.create_folder().unwrap_or_else(|error| {
            eprint!("{error}");
            exit(0);
        });

        // Export images
        let mut buffer: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(0, 0);
        for output_file in app_store_outputs { 
            if buffer.height() != output_file.size {
                buffer = imageops::resize(&app_store_image, output_file.size, output_file.size, imageops::FilterType::CatmullRom); 
            }
            self.save(&buffer, &output_file.name, &output_file.format);
        }
        println!("Finish");
    }
    
    fn save(&self, output: &ImageBuffer<Rgba<u16>, Vec<u16>>, file_name: &str, format: &str) { 
        let output_file_path = self.output_path.to_string() + "/" + file_name + "." + format;
        output.save(&output_file_path).unwrap_or_else(|error| {
            eprintln!("Fail to save the file!: {error}");
        });
    }

    fn create_folder(&self) -> std::io::Result<()> {
        let path = Path::new(&self.output_path);
        if !path.exists() {
            create_dir_all(path)?;
        }
        Ok(())
    }
    
    fn check_input_image(&mut self) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>, &str> {
        let img = open(&self.input_path).unwrap().into_rgba16();
        let (x, y) = img.dimensions();
        if x != y {
            return Err("image is not square");
        }
        if x < 1024 || y < 1024 {
            return Err("Image resolution must be grater or equal to 1024 x 1024");
        } 
        if x > 1024 || y > 1024 {
            let resize_image = imageops::resize(&img, 1024, 1024, imageops::FilterType::CatmullRom);
            return Ok(resize_image)
        }
        Ok(img)
    }

    fn rounded_corners(&self, img: &ImageBuffer<Rgba<u16>, Vec<u16>>, radius: u32) -> ImageBuffer<Rgba<u16>, Vec<u16>> {
        let mut output: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(1024, 1024);
        if radius == 0 {
            let _ = output.copy_from(img, 0, 0);
            return output
        }
        let center = (1024 / 2, 1024 / 2); 
        let top_left = (radius, radius);
        let top_right = (1024 - radius, radius);
        let bottom_left = (radius, 1024 - radius);
        let bottom_right = (1024 - radius, 1024 - radius);

        for (x, y, pixel) in img.enumerate_pixels() {
            let point = (x, y);
            let distance_center = self.find_distance(point, center);
            let distance_top_left = self.find_distance(point, top_left);
            let distance_top_right = self.find_distance(point, top_right);
            let distance_bottom_left = self.find_distance(point, bottom_left);
            let distance_bottom_right = self.find_distance(point, bottom_right);

            if distance_center > self.find_distance((0, radius), center) && distance_top_left > radius && distance_top_right > radius && distance_bottom_left > radius && distance_bottom_right > radius {
                continue;
            }
            output.put_pixel(x, y, *pixel);
        }

        output
    }

    fn insert_transparent_border(&self, img: &ImageBuffer<Rgba<u16>, Vec<u16>>) -> ImageBuffer<Rgba<u16>, Vec<u16>> {
        let target_size = 824;
        let resized_image = imageops::resize(img, target_size, target_size, imageops::FilterType::CatmullRom);
        let mut output: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::new(1024, 1024);
        let start_index = 100;
        for (x, y, pixel) in resized_image.enumerate_pixels() {
            output.put_pixel(start_index + x, start_index + y, *pixel);
        }

        output
    }

    fn find_distance(&self, point_a: (u32, u32), point_b: (u32, u32)) -> u32 {
        let i_point_a: (i32, i32) = (point_a.0 as i32, point_a.1 as i32);
        let i_point_b: (i32, i32) = (point_b.0 as i32, point_b.1 as i32);
        let x: i32 = (i_point_a.0 - i_point_b.0) * (i_point_a.0 - i_point_b.0); 
        let y: i32 = (i_point_a.1 - i_point_b.1) * (i_point_a.1 - i_point_b.1);
        ((x + y) as f32).sqrt() as u32
    }

}

