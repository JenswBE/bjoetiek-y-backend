use std::path::PathBuf;

use actix::{Actor, Context, Handler, Message};
use failure::Error;

pub struct ImageActor {
    path: PathBuf,
}

impl Actor for ImageActor {
    type Context = Context<Self>;
}

impl ImageActor {
    pub fn new(images_path: PathBuf) -> Self {
        if !images_path.exists() {
            panic!("Provided images path does not exist")
        }

        Self { path: images_path }
    }
}

pub struct UploadImage {
    pub id: uuid::Uuid, // ID of the owner (product, manufacturer, ...)
    pub data: Vec<u8>,
}

impl Message for UploadImage {
    type Result = Result<(), Error>;
}

impl Handler<UploadImage> for ImageActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: UploadImage, _: &mut Self::Context) -> Self::Result {
        // Generate image path
        let mut image_path = self.path.clone();
        image_path.push(msg.id.to_string());
        image_path.set_extension("png");

        // Load file into VIPS and write as png
        let image = libvips::VipsImage::new_from_buffer(&msg.data, "").map_err(Error::from)?;
        image
            .image_write_to_file(image_path.to_str().unwrap())
            .map_err(Error::from)
    }
}
