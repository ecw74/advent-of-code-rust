use image::ImageFormat;
use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use askama::Template;

// Define a template structure for generating an image server
#[derive(Template)]
#[template(path = "image_handler.rs", escape = "none")]
pub struct ImageServerTemplate {
    build_dir: String, // Directory where processed images are stored
    images: Vec<String>, // List of image filenames
}

fn main() {
    // Retrieve the output directory for the build process
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let build_dir = Path::new(&out_dir).join("build"); // Define the build directory path
    let crate_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let assets_dir = Path::new(&crate_dir).join("assets"); // Path to the assets directory
    let output_rs_path = Path::new(&crate_dir).join("src").join("image_handler.rs"); // Output path for the generated Rust file

    // Notify Cargo to rerun the build if the template file changes
    println!("cargo:rerun-if-changed=templates/image_handler.rs");

    // Process the images in the assets directory
    process_images(&assets_dir, &build_dir);

    // Generate the image handler Rust file from the template
    generate_image_handler(&output_rs_path, &build_dir);

    // Retrieve build date from an environment variable or generate it using `date`
    let build_date = env::var("BUILD_DATE").unwrap_or_else(|_| {
        let output = Command::new("date")
            .arg("+%Y-%m-%dT%H:%M:%S")
            .output()
            .expect("Failed to execute date command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    // Retrieve the Git commit hash from an environment variable or execute the `git` command
    let commit_hash = env::var("COMMIT_HASH").unwrap_or_else(|_| {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    // Retrieve the short version of the Git commit hash
    let commit_hash_short = env::var("COMMIT_HASH_SHORT").unwrap_or_else(|_| {
        let output = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    // Set environment variables for the Rust compiler
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=COMMIT_HASH_SHORT={}", commit_hash_short);

    // Execute additional embedded build system steps
    embuild::espidf::sysenv::output();
}

fn process_images(assets_dir: &Path, build_dir: &Path) {
    // Ensure the build output directory exists
    if build_dir.exists() {
        fs::remove_dir_all(build_dir).expect("Failed to clean build directory");
    }
    fs::create_dir_all(build_dir).expect("Failed to create build directory");

    // Process all image files in the assets directory
    for entry in fs::read_dir(assets_dir).expect("Failed to read assets directory") {
        let asset_dir = entry.expect("Failed to read directory entry").path();
        if asset_dir.is_dir() {
            for img_entry in fs::read_dir(&asset_dir).expect("Failed to read year directory") {
                let img_path = img_entry.expect("Failed to read image entry").path();
                if img_path.extension().map_or(false, |ext| ext == "webp") {
                    println!("cargo:rerun-if-changed={}", img_path.display());
                    resize_image(&img_path, build_dir); // Speichern direkt im Build-Ordner
                }
            }
        }
    }
}

// Function to resize an image to specific dimensions and save it in WebP format
fn resize_image(input_path: &Path, output_dir: &Path) {
    let mut output_path = output_dir.join(input_path.file_name().unwrap()); // Output path for the resized image
    output_path.set_extension("avif");

    // Open the image from the input path
    let img = image::open(input_path).expect("Failed to open image");
    let resized = img.resize_exact(200, 200, image::imageops::FilterType::Lanczos3); // Resize image to 182x182 pixels

    // Save the resized image in WebP format
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    resized
        .write_to(&mut output_file, ImageFormat::Avif)
        .expect("Failed to save resized image");
}

// Function to generate the Rust file for handling images
fn generate_image_handler(output_rs_path: &Path, build_dir: &Path) {
    let mut images = Vec::new(); // Initialize a vector to store image file names

    // Iterate through all entries in the build directory
    for img_entry in fs::read_dir(build_dir).expect("Failed to read build directory") {
        let img_path = img_entry.expect("Failed to read image entry").path();
        if img_path.extension().map_or(false, |ext| ext == "avif") { // Filter WebP images
            let image_name = img_path.file_name().unwrap().to_str().unwrap();
            images.push(image_name.to_string()); // Add the image name to the list
        }
    }

    // Render the template with the build directory and image list
    let build_dir_str = build_dir.to_str().unwrap_or_default().to_string();
    let template = ImageServerTemplate {
        build_dir: build_dir_str,
        images,
    };
    let rendered = template.render().expect("Failed to render template");

    // Write the rendered template to the output file
    fs::write(output_rs_path, rendered).expect("Failed to write generated code");
}
