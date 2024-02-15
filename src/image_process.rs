use core::panic;
use std::process::exit;

use image::{ImageBuffer, open, Rgba, GenericImage};

#[derive(Debug)]
pub struct ImageProcess {
    input_path: String,
    output_path: String,
    width: u32,
    height: u32,
}

impl ImageProcess {
    pub fn new(input_path: String, output_path: String) -> Self {
        let input = input_path.replace("\n", "");
        let output = output_path.replace("\n", "");
        if input.is_empty() || output.is_empty() {
            panic!("path cannot be empty")
        }
        Self { 
            input_path: input,
            output_path: output,
            width: 0,
            height: 0
        }
    } 

    pub fn start(&mut self) {
        let image = self.check_input_image().unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(0);
        });
        let mut output = ImageBuffer::new(self.width, self.height);
        self.rounded_corners(&image, 186, &mut output);
        output.save(&self.output_path).expect("Fail to save the file!");
    }
    
    fn check_input_image(&mut self) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>, &str> {
        let img = open(&self.input_path).expect("File not found").into_rgba16();
        let (x, y) = img.dimensions();
        if x < 1024 || y < 1024 {
            return Err("Image resolution must be grater or equal to 1024 x 1024");
        }
        if x != y {
            return Err("image is not square");
        }
        self.width = x;
        self.height = y;
        Ok(img)
    }

    fn rounded_corners(&self, img: &ImageBuffer<Rgba<u16>, Vec<u16>>, radius: u32, output: &mut ImageBuffer<Rgba<u16>, Vec<u16>>) {
        if radius == 0 {
            let _ = output.copy_from(img, 0, 0);
            return;
        }
        let center = (self.width / 2, self.width / 2); 
        let top_left_center = (radius, radius);
        let top_right_center = (self.width - radius, radius);
        let bottom_left_center = (radius, self.width - radius);
        let bottom_right_center = (self.width - radius, self.width - radius);

        for (x, y, pixel) in img.enumerate_pixels() {
            let point = (x, y);
            let distance_center = self.find_distance(point, center);
            let distance_top_left = self.find_distance(point, top_left_center);
            let distance_top_right = self.find_distance(point, top_right_center);
            let distance_bottom_left = self.find_distance(point, bottom_left_center);
            let distance_bottom_right = self.find_distance(point, bottom_right_center);

            if distance_center > self.find_distance((0, radius), center) && distance_top_left > radius && distance_top_right > radius && distance_bottom_left > radius && distance_bottom_right > radius {
                continue;
            }
            output.put_pixel(x, y, *pixel);
        }
    }

    fn find_distance(&self, point_a: (u32, u32), point_b: (u32, u32)) -> u32 {
        let i_point_a: (i32, i32) = (point_a.0 as i32, point_a.1 as i32);
        let i_point_b: (i32, i32) = (point_b.0 as i32, point_b.1 as i32);
        let x: i32 = (i_point_a.0 - i_point_b.0) * (i_point_a.0 - i_point_b.0); 
        let y: i32 = (i_point_a.1 - i_point_b.1) * (i_point_a.1 - i_point_b.1);
        ((x + y) as f32).sqrt() as u32
    }

}

