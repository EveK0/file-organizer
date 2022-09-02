#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use nwg::NativeUi;
use std::env;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    layout: nwg::GridLayout,
    text: nwg::Label,
    segoe_font: nwg::Font,
    segoe_font2: nwg::Font,
    roboto_mono: nwg::Font,
    star_stop: nwg::Button,
}

impl BasicApp {
    fn say_goodbye(&self) {
        // nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;
            // Controls
            nwg::Window
                ::builder()
                .flags(
                    nwg::WindowFlags::WINDOW |
                        nwg::WindowFlags::VISIBLE |
                        nwg::WindowFlags::MINIMIZE_BOX
                )
                .size((600, 400))
                .position((300, 300))
                .title("FO - File Organizer")
                .build(&mut data.window)?;

            nwg::Font::builder().size(11).family("Segoe UI").weight(1).build(&mut data.segoe_font)?;

            nwg::Font
                ::builder()
                .size(24)
                .family("Segoe UI")
                .weight(1)
                .build(&mut data.segoe_font2)?;
            nwg::Font
                ::builder()
                .size(24)
                .family("Roboto Mono")
                .weight(2)
                .build(&mut data.roboto_mono)?;
            nwg::Label
                ::builder()
                .text("Choose a directory for organize")
                .size((190, 90))
                .font(Some(&data.roboto_mono))
                .position((210, 80))
                .parent(&data.window)
                .build(&mut data.text)?;

            nwg::Button
                ::builder()
                .text("Browse")
                .size((138, 38))
                .position((230, 200))
                .parent(&data.window)
                .font(Some(&data.segoe_font2))
                .build(&mut data.star_stop)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };
            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnWindowClose => {
                            if &handle == &ui.window {
                                BasicApp::say_goodbye(&ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(
                nwg::full_bind_event_handler(&ui.window.handle, handle_events)
            );

            // Layouts
            nwg::GridLayout::builder().parent(&ui.window).spacing(0).build(&ui.layout)?;

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();

            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
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

            let videos = ["mp4", "webm", "ogv", "avi", "mov", "flv", "wmv", "mpg", "mpeg", "3gp"];

            let audio = ["mp3", "wav", "ogg", "flac", "aac", "wma", "m4a"];

            let documents = [
                "txt",
                "doc",
                "docx",
                "xls",
                "xlsx",
                "ppt",
                "pptx",
                "pdf",
                "odt",
                "ods",
                "odp",
                "rtf",
                "csv",
                "zip",
                "rar",
                "7z",
                "tar",
                "gz",
                "bz2",
                "xz",
                "z",
                "tgz",
                "7z",
                "jfif",
                "htm",
                "html",
                "xml",
                "json",
                "yml",
                "yaml",
                "toml",
                "md",
                "markdown",
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
        } else {
            println!("Error! This is not a file");
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