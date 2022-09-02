use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use std::path::PathBuf;
fn main() {
    // GET THE PATH TO THE USER'S HOME DIRECTORY
    let username: String = env::var("USERPROFILE").unwrap();

    // GET THE DIRECTORY PATH
    let paths: ReadDir = fs::read_dir(Path::new(&username).join("Downloads")).unwrap();

    // LISTING THE DIRECTORY PATH
    println!("I'm Running!");
    for path in paths {
        let temp = path.unwrap().path();
        if temp.is_file() {
            let images = ["jpg", "png", "gif", "bmp", "jpeg", "webp", "svg", "ico"];

            let videos = [
                "mp4", "webm", "ogv", "avi", "mov", "flv", "wmv", "mpg", "mpeg", "3gp",
            ];

            let audio = ["mp3", "wav", "ogg", "flac", "aac", "wma", "m4a"];

            let documents = [
                "txt", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "pdf", "odt", "ods", "odp",
                "rtf", "csv", "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "z", "tgz", "7z",
                "jfif", "htm", "html", "xml", "json", "yml", "yaml", "toml", "md", "markdown",
                "ini",
            ];

            let application = ["msi", "exe"];
            let path_video = Path::new(&username).join("Downloads").join("Video");
            let path_image = Path::new(&username).join("Downloads").join("Image");
            let path_document = Path::new(&username).join("Downloads").join("Documents");
            let path_application = Path::new(&username).join("Downloads").join("Application");
            let path_audio = Path::new(&username).join("Downloads").join("Audio");
            if images.contains(&temp.extension().unwrap().to_str().unwrap()) {
                create_directory(&path_image);
                move_file(&temp, &path_image.join(temp.file_name().unwrap()));
            } else if videos.contains(&temp.extension().unwrap().to_str().unwrap()) {
                create_directory(&path_video);
                move_file(&temp, &path_video.join(temp.file_name().unwrap()));
            } else if audio.contains(&temp.extension().unwrap().to_str().unwrap()) {
                create_directory(&path_audio);
                move_file(&temp, &path_audio.join(temp.file_name().unwrap()));
            } else if documents.contains(&temp.extension().unwrap().to_str().unwrap()) {
                create_directory(&path_document);
                move_file(&temp, &path_document.join(temp.file_name().unwrap()));
            } else if application.contains(&temp.extension().unwrap().to_str().unwrap()) {
                create_directory(&path_application);
                move_file(&temp, &path_application.join(temp.file_name().unwrap()));
            } else {
                println!("Unknown");
            }
        }
        else {
            println!("Error! This is not a file")
        }
    }
}





fn create_directory(path: &Path) {
    if !path.exists() {
        std::fs::create_dir(path).unwrap();
    }
}
fn move_file(root: &Path, path: &Path) {
    fs::rename(root, path).unwrap();
}
