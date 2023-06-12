use chrono::{DateTime, Local};
use image::{imageops, GenericImageView};
use std::env;
use std::fs::metadata;
use std::path::Path;

trait ImageMetadata {
    fn print_metadata(&self, image_path: &str);
    fn compress(
        &self,
        img_width: u32,
        img_height: u32,
        output_path: &str,
    ) -> Result<(), image::ImageError>;
}
struct DynamicImageWrapper<'a>(&'a image::DynamicImage);

impl<'a> ImageMetadata for DynamicImageWrapper<'a> {
    fn print_metadata(&self, image_path: &str) {
        let original_file_name = Path::new(image_path).file_name().unwrap().to_string_lossy();

        let last_created = metadata(image_path)
            .and_then(|metadata| metadata.modified())
            .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);

        let last_created_dt: DateTime<Local> = last_created.into();

        println!("Metadata of the image:");

        println!("Image Path: {}", image_path);

        println!("File Name: {}", original_file_name);

        println!("Last created: {}", last_created_dt);

        println!("Dimensions: {}x{}", self.0.width(), self.0.height());
        println!("Color Type: {:?}", self.0.color());

        println!(
            "Size of the Image: {} bytes",
            metadata(image_path).unwrap().len()
        );

        println!();
    }

    fn compress(
        &self,

        img_width: u32,

        img_height: u32,

        output_path: &str,
    ) -> Result<(), image::ImageError> {
        let compressed_image =
            self.0
                .resize_exact(img_width, img_height, imageops::FilterType::Lanczos3);

        compressed_image.save(output_path)?;
        let wrapper = DynamicImageWrapper(&compressed_image);
        wrapper.print_metadata(output_path);
        println!("Image saved at: {}", output_path);
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: image_utility <input_image> <img_width> <img_height> <output_path>");
        return;
    }

    let input_image_path = &args[1];

    let img_width = match args[2].parse() {
        Ok(size) => size,
        Err(err) => {
            println!("Invalid compressed size provided: {}", err);

            return;
        }
    };

    let img_height = match args[3].parse() {
        Ok(size) => size,
        Err(err) => {
            println!("Invalid compressed size provided: {}", err);
            return;
        }
    };
    let output_path = &args[4];
    let input_image = match image::open(input_image_path) {
        Ok(img) => {
            println!("Image loaded successfully");
            let wrapper = DynamicImageWrapper(&img);
            wrapper.print_metadata(input_image_path);
            img
        }
        Err(err) => {
            println!("Failed to open image: {}", err);
            return;
        }
    };

    let wrapper = DynamicImageWrapper(&input_image);

    if let Err(err) = wrapper.compress(img_width, img_height, output_path) {
        println!("Failed to compress and save image: {}", err);
    }
}