use image::{ImageBuffer, open, Rgba, GenericImage};

fn main() {
    let img = open("src/icon.png").expect("File not found").into_rgba16();
    let (x, y) = img.dimensions();
    if x < 1024 || y < 1024 {
        println!("Image resolution must be grater or equal to 1024 x 1024");
        return;
    }
    if x != y {
        println!("image is not square");
        return;
    }
    let mut output = ImageBuffer::new(x, y);
    rounded_corners(&img, x, 186, &mut output);
    output.save("src/output.png").expect("Fail to save the file!");
}

fn rounded_corners(img: &ImageBuffer<Rgba<u16>, Vec<u16>>, width: u32, radius: u32, output: &mut ImageBuffer<Rgba<u16>, Vec<u16>>) {
    if radius == 0 {
        let _ = output.copy_from(img, 0, 0);
        return;
    }
    let center = center_coordinate(0, 0, width);
    let top_left_center = (radius, radius);
    let top_right_center = (width - radius, radius);
    let bottom_left_center = (radius, width - radius);
    let bottom_right_center = (width - radius, width - radius);

    for (x, y, pixel) in img.enumerate_pixels() {
        let point = (x, y);
        let distance_center = find_distance(point, center);
        let distance_top_left = find_distance(point, top_left_center);
        let distance_top_right = find_distance(point, top_right_center);
        let distance_bottom_left = find_distance(point, bottom_left_center);
        let distance_bottom_right = find_distance(point, bottom_right_center);

        if distance_center > find_distance((0, radius), center) && distance_top_left > radius && distance_top_right > radius && distance_bottom_left > radius && distance_bottom_right > radius {
            continue;
        }
        output.put_pixel(x, y, *pixel);
    }
}

fn center_coordinate(x: u32, y: u32, width: u32) -> (u32, u32) {
    (x + width / 2, y + width / 2)
}

fn find_distance(point_a: (u32, u32), point_b: (u32, u32)) -> u32 {
    let i_point_a: (i32, i32) = (point_a.0 as i32, point_a.1 as i32);
    let i_point_b: (i32, i32) = (point_b.0 as i32, point_b.1 as i32);
    let x: i32 = (i_point_a.0 - i_point_b.0) * (i_point_a.0 - i_point_b.0); 
    let y: i32 = (i_point_a.1 - i_point_b.1) * (i_point_a.1 - i_point_b.1);
    ((x + y) as f32).sqrt() as u32
}

