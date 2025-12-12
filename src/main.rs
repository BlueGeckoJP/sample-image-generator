mod pattern;

use std::{fs, path::Path};

use image::ImageBuffer;

use crate::pattern::{Pattern, checker};

const OUTPUT_DIR: &str = "output";

fn prepare_output_directory() {
    let output_dir = Path::new(OUTPUT_DIR);
    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Failed to create output directory");
    }
}

fn main() {
    prepare_output_directory();

    let patterns: Vec<Box<dyn Pattern>> = vec![Box::new(checker::CheckerPattern {})];

    for pattern in patterns {
        let (width, height) = (512, 512);
        let data = pattern.generate(width, height);
        let mut image_buffer = ImageBuffer::new(width, height);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let index = (y * width + x) as usize;
            let value = data[index];
            *pixel = image::Rgb([value * 255, value * 255, value * 255]);
        }

        let type_name = pattern.type_name();
        let filename = format!(
            "./{}/output_{}.png",
            OUTPUT_DIR,
            type_name
                .replace("sample_image_generator::", "")
                .replace("::", "_")
                .replace(" ", "_")
        );
        println!("Saving image to {}", filename);
        image_buffer
            .save(&filename)
            .expect("Failed to save the image");
    }
}
