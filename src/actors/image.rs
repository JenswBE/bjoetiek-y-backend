use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

use actix::{Actor, Context, Handler, Message};

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

        // Write file
        let mut f = std::fs::File::create(image_path)?;
        f.write_all(&msg.data)
    }
}
