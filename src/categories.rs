use std::path::Path;

pub fn get_category(path:&Path) -> &str {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => match ext.to_ascii_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "webp" => "Images",
            "mp4" | "mkv" => "Videos",
            "pdf" | "docx" => "Documents",
            _ => "Others",
        },
        None => "Others",
    }
}