use core::panic;
use std::{fs::create_dir_all, path::Path, process::exit};

use image::{open, DynamicImage, GenericImage, ImageBuffer, Rgba};

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
        let mut output = output_path.replace("\n", "");
        if input.is_empty() || output.is_empty() {
            panic!("path cannot be empty")
        }
        output = output + "/output";
        Self { 
            input_path: input,
            output_path: output,
            width: 0,
            height: 0
        }
    } 

    pub fn run(&mut self) {
        let image = self.check_input_image().unwrap_or_else(|error| {
            eprintln!("{}", error);
            exit(0);
        });
        let mut rounded_corners_image = ImageBuffer::new(self.width, self.height);
        self.rounded_corners(&image, 186, &mut rounded_corners_image);
        let dynamic = DynamicImage::ImageRgba16(rounded_corners_image);
        let app_store_outputs = OutputFormat::app_store_outputs();

        // Create folder
        self.create_folder().unwrap_or_else(|error| {
            eprint!("{error}");
            exit(0);
        });

        // Export images
        for output_file in app_store_outputs {
           let output = dynamic.resize(output_file.size, output_file.size, image::imageops::FilterType::CatmullRom); 
           self.save(&output, &output_file.name, &output_file.format);
        }
        println!("Finish");
    }
    
    fn save(&self, output: &DynamicImage, file_name: &str, format: &str) { 
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

struct OutputFormat {
    name: String,
    size: u32,
    format: String,
}

impl OutputFormat {
    fn new(name: String, size: u32, format: String) -> Self {
        Self { name, size, format }
    }

    fn app_store_outputs() -> Vec<OutputFormat> {
        let format: String = "png".to_string();
        vec![
            Self::new("1024".to_string(), 1024, format.clone()),
            Self::new("512@2x".to_string(), 1024, format.clone()),
            Self::new("512".to_string(), 512, format.clone()),
            Self::new("256@2x".to_string(), 256, format.clone()),
            Self::new("256".to_string(), 256, format.clone()),
            Self::new("128@2x".to_string(), 256, format.clone()),
            Self::new("128".to_string(), 128, format.clone()),
            Self::new("32@2x".to_string(), 64, format.clone()),
            Self::new("32".to_string(), 32, format.clone()),
            Self::new("16@2x".to_string(), 32, format.clone()),
            Self::new("16".to_string(), 16, format.clone()),
        ]
    }
}
