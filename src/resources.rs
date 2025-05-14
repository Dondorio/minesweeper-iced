use iced::widget::image::Handle;
use include_dir::{Dir, include_dir};

pub static RESOURCES: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

pub fn get_image_handle(name: impl Into<String>) -> Handle {
    Handle::from_bytes(RESOURCES.get_file(name.into()).unwrap().contents())
}
