use image::{ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

#[tauri::command]
pub fn convert(path: String, to_format: String, output_dir: String) -> Result<(), String> {
    println!(
        "received request to convert:\n\tpath:{}\n\tto_format:{}\n\toutput_dir:{}\n",
        path, to_format, output_dir
    );

    let file_stem = Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("File name couldn't be parsed from given path")?;

    println!("reading image '{}' from: {}", file_stem, path);
    let img = ImageReader::open(&path)
        .map_err(|e| {
            eprintln!("Error when opening file: {}", e);
            "Couldn't open file"
        })?
        .decode()
        .map_err(|e| {
            eprintln!("Error when decoding file: {}", e);
            "Couldn't decode file"
        })?;

    let format = match to_format.as_str() {
        "jpg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "webp" => ImageFormat::WebP,
        _ => return Err("Converting to format not supported".to_string()),
    };
    let mut output_path = PathBuf::from(output_dir);
    output_path.push(file_stem);
    output_path.set_extension(to_format);

    println!(
        "finna save at '{}' with format: {}",
        output_path.to_str().unwrap_or_default(),
        format.to_mime_type()
    );
    img.save_with_format(output_path, format).map_err(|e| {
        eprintln!("Error when saving file: {}", e);
        "Couldn't save file"
    })?;
    println!("done saved image");

    Ok(())
}
